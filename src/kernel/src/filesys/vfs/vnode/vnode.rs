use alloc::sync::Arc;

use crate::device::{BlockDevice, CharDevice, Device};
use crate::filesys::mount::MountPoint;
use crate::filesys::{self, vfs::vnode::interface, AccessFlags, DirectoryEntry, FileDescriptor};
use crate::ipc::socket::SocketAddress;
use crate::time::TimeSpec;

/// Types of V-nodes.
#[derive(Debug)]
pub enum VnodeKind {
    /// Regular file.
    DirectoryEntry(dyn interface::FileInterface),
    /// Directory.
    Directory(dyn interface::DirectoryInterface),
    /// Symbolic link.
    SymbolicLink(Arc<Vnode>),
    /// Character device.
    CharDevice(Arc<CharDevice>),
    /// Block device.
    BlockDevice(Arc<BlockDevice>),
    /// Socket.
    Socket(dyn interface::SocketInterface),
    /// Super-block of file system.
    SuperBlock(dyn interface::FileSystemInterface),
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

/// Statistics for a V-node provided through [`VnodeInterface::stats`].
pub struct VnodeStats {
    /// Number of hard links.
    hard_link_count: usize,
    /// Total size, in bytes.
    size: usize,
    /// Size of each block.
    block_size: usize,
}
