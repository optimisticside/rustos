use crate::device::BlockDevice;
use self::vnode::Vnode;

pub mod ext2;
pub mod minix;
pub mod vnode;

/// Represents a file-system.
pub trait FileSystem {
    fn read(device: BlockDevice, vnode: &Vnode, buffer: &[u8], offset: u32) -> Result<u32, FsError>;
    fn write(
        device: BlockDevice,
        vnode: &Vnode,
        buffer: &[u8],
        offset: u32
    ) -> Result<u32, FsError>;
}
