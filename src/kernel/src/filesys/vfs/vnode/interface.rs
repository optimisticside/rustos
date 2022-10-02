use crate::filesys::{AccessFlags, DirectoryEntry, FileDescriptor, FileSysError, Vnode};
use crate::ipc::socket::SocketAddress;

/// Callbacks that are shared among all V-nodes.
pub trait VnodeInterface: Send + Sync {
    /// Link the node to a directory entry.
    fn link(vnode: &Vnode, name: &str) -> Result<(), FileSysError>;
    /// Unlink the node from a directory entry. Note that the caller is responsible for
    /// de-allocating the V-node if the count reaches 0.
    fn unlink(vnode: &Vnode, name: &str) -> Result<(), FileSysError>;
    /// Get information about the V-node.
    fn stats(vnode: &Vnode) -> Result<VnodeStats, FileSysError>;
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
}

/// Callbacks for directories.
pub trait DirectoryInterface: VnodeInterface {
    /// Allows caller to read entries of the directory, by providing the next entry in the
    /// directory (or the first if the user does not provide a directory-entry).
    fn read_dir(vnode: &Vnode, dir_entry: Option<&DirectoryEntry>) -> Result<Option<Arc<DirEntry>>, FileSysError>;
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
    /// Rename the file. Defaults to reading the entire file, deleting it, and creating and writing
    /// to a new file with the new name.
    fn rename(vnode: &Vnode, dir_entry: &DirectoryEntry, name: &str) -> Result<(), FileSysError>;
}

/// Callbacks for sockets.
pub trait SocketInterface: FileInterface {
    /// Listen for connections on a socket.
    fn listen(vnode: &Vnode, backlog: usize) -> Result<(), FileSysError>;
    /// Tell the socket what port we want to serve.
    fn bind(vnode: &Vnode, addr: SocketAddress, length: usize) -> Result<(), FileSysError>;
    /// Connect socket to specific port on a remote system. This is usually done right after the
    /// socket is created.
    fn connect(vnode: &Vnode, addr: SocketAddress, length: usize) -> Result<(), FileSysError>;
    /// Accept a connection request from a client (made through [`SocketInterface::connect`]).
    fn accept(vnode: &Vnode, addr: Option<&mut SocketAddress>) -> Result<Arc<Socket>, FileSysError>;
}

/// Callbacks for file-system drivers.
pub trait FileSystemInterface: FileInterface + DirectoryInterface {}
