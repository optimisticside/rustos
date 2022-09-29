use crate::devices::{Device, BlockDevice, CharDevice};
use crate::filesys::{self, mount::MountPoint};

/// Types of V-nodes.
#[derive(Debug)]
pub enum VnodeKind {
    /// Regular file.
    DirectoryEntry(dyn FileInterface),
    /// Directory.
    Directory(dyn DirectoryInterface),
    /// Symbolic link.
    SymbolicLink(Arc<Vnode>),
    /// Character device.
    CharDevice(Arc<CharDevice>),
    /// Block device.
    BlockDevice(Arc<BlockDevice>),
    /// Socket.
    Socket(dyn SocketInterface),
    /// Super-block of file system.
    SuperBlock(dyn FileSystemInterface),
}

/// Types of V-node data. This is data that is specific to the V-node's file-system.
pub enum VnodeFileSystem {
    /// There is no file-system. This is the case for V-nodes that do not map to files or
    /// directories and are instead the devices themselves or something else (like a Socket).
    Null,
    /// Minix file-system.
    Minix(filesys::minix::Inode),
}

/// A V-node is the focus of file activity on UNIX system. There is one allocated for every active
/// file, directory, mounted-file, and the file-system's root.
pub struct Vnode {
    /// Number of references to node. Will be reallocated if this reaches 0.
    pub ref_count: usize,
    /// Type of V-node. All interface-operations and other kind-specific data are stored here.
    pub kind: VnodeKind,
    /// Data specific to the file-system that the V-node is physically stored on.
    pub file_sys: VnodeFileSystem,
    /// Device that the V-node is stored on.
    pub device: Arc<dyn Device>,
}

/// Callbacks that are shared among all V-nodes.
pub trait VnodeInterface: Send + Sync {
     /// Link the node to a directory entry.
    fn link(vnode: &Vnode, name: &str) -> Result<(), FileSysError>;
    /// Unlink the node from a directory entry. Note that the caller is responsible for
    /// de-allocating the V-node if the count reaches 0.
    fn unlink(vnode: &Vnode, name: &str) -> Result<(), FileSysError>;
    /// Perform an I/O control call on the device (for device-specific things).
    fn io_control(vnode: &Vnode, operation: usize, buffer: &[u8]) -> Result<(), FileSysError>;
}

/// Callbacks that can be run for directory entries (files).
pub trait FileInterface: VnodeInterface {
    /// Open a file with the given file-descriptor.
    fn open(vnode: &Vnode, file_desc: &FileDescriptor) -> Result<(), FileSysError>;
    /// Close the file of the given file-descriptor.
    fn close(vnode: &Vnode, file_desc: &FileDescriptor) -> Result<(), FileSysError>;
    /// Read from the file at the given offset, into the given buffer.
    fn read(vnode: &Vnode, offset: usize, buffer: &[u8]) -> Result<usize, FileSysError>;
    /// Write to a node at the given offset, from the given buffer.
    fn write(vnode: &Vnode, offset: usize, buffer: &[u8]) -> Result<usize, FileSysError>;
    /// Rename the file.
    fn rename(vnode: &Vnode, name: &str) -> Result<(), FileSysError>;
}

/// Callbacks for directories.
pub trait DirectoryInterface: VnodeInterface {
    /// Allows caller to read entries of the directory, by providing the next entry in the
    /// directory (or the first if the user does not provide a directory-entry).
    fn read_dir(vnode: &Vnode, dir_entry: Option<DirEntry>) -> Result<Option<Arc<DirEntry>>, FileSysError>;
    /// Create a directory with the provided name.
    fn make_dir(vnode: &Vnode, name: &str) -> Result<Vnode, FileSysError>;
    /// Remove a directory given its name.
    fn remove_dir(vnode: &Vnode, name: &str) -> Result<(), FileSysError>;
    /// Open a new file in the directory, and create one if it does not exist.
    fn open(vnode: &Vnode, name: &str, flags: AccessFlags) -> Result<FileDescriptor, FileSysError>;
    /// Close a held file-descriptor.
    fn close(vnode: &Vnode, file_desc: &FileDescriptor) -> Result<(), FileSysError>;
    /// Create a custom V-node inside the directory. This is usually used to create devices, such
    /// as in a `/dev` directory.
    fn make_node(vnode: &Vnode, kind: VnodeKind) -> Result<(), FileSysError>;
}

/// Callbacks for file-system drivers.
pub trait FileSystemInterface: FileInterface + DirectoryInterface {}
