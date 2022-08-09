use std::fs;

use std::error::Error;
use std::ffi::OsString;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Call a function for every file that is a descendant of the given directory.
fn visit_dirs(directory: &Path, callback: &mut dyn FnMut(&DirEntry)) -> std::io::Result<()> {
    if directory.is_dir() {
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_dir() {
                visit_dirs(&entry_path, callback)?;
            } else {
                callback(&entry);
            }
        }
    }

    Ok(())
}

/// Retrieve a file's name from its path.
fn name_from_path<'a>(path: &'a PathBuf) -> &'a str {
    let os_name = path.file_name().expect("Unable to get file name");
    os_name.to_str().expect("Invalid encoding for file name")
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut include_files = vec![];

    // Assemble all real files or files that end in .inc so that they can be included when
    // compiling source files.
    visit_dirs(Path::new("src"), &mut |entry| {
        let path = entry.path();

        match path.extension() {
            Some(extension) if extension.eq(&OsString::from("real")) => {
                let file_name = name_from_path(&path);
                // TODO: Finish this
            },

            Some(extension) if extension.eq(&OsString::from("inc")) => {
                let path_str = path.to_str().expect("Invalid encoding for file path");
                include_files.push(path_str.to_string())
            },

            _ => (),
        }
    });

    // Assemble any other files.
    visit_dirs(Path::new("src"), &mut |entry| {
        let path = entry.path();

        match path.extension() {
            Some (extension) if extension.eq(&OsString::from("asm")) => {
                let file_name = name_from_path(&path);
                let mut build = nasm_rs::Build::new();

                build
                    .file(&path)
                    .flag("-felf64")
                    .target("x86_64-unknown-none");
                
                for include in &include_files {
                    build.include(include);
                }

                build
                    .compile(file_name)
                    .expect("Unable to compile");

                // Link as a static library.
                println!("cargo:rustc-link-lib=static={}", file_name);
            },

            _ => (),
        }
    });

    Ok(())
}
