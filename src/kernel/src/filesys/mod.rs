pub use self::error::*;
pub use self::vfs::*;

pub mod ext2;
pub mod error;
pub mod minix;
pub mod vfs;

/// Types of file-systems. Used to store device-specific data in V-nodes.
pub enum FileSysType {
    Minix(self::minix::Inode),
}
