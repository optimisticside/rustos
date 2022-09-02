/// An I-node on the EXT-2 file system is, though of the same name, not the same as an I-node in
/// the virtual file system. This is why all BSD operating systems call i-nodes in the VFS v-nodes
/// instead.
#[repr(C, packed)]
pub struct Inode {
    /// Type and permissions.
    mode: u16,
    /// User ID.
    user_id: u16,
    /// Lower 32 bits of the size in bytes.
    size: u32,
    /// Last access time.
    access_time: u32,
    /// Creation time.
    create_time: u32,
    /// Last modification time.
    modify_time: u32,
    /// Group ID.
    group_id: u16,
    /// Number of hard links (directory entries) to this inode. Data blocks are unallocated when
    /// this reaches 0.
    link_count: u16,
    /// Number of disk sectors in use by the inode, not including the i-node itself or any
    /// directory entries.
    block_count: u16,
    /// Flags.
    flags: u32,
    reserved: u32,
    /// Disk block pointers.
    /// Entires 1-11: Direct block pointers.
    /// Entry 12: Singly indirect block pointer.
    /// Entry 13: Doubly indirect block pointer.
    /// Entry 14: Triply indirect block pointer.
    blocks: [u32; 15],
    /// Generation number (mostly used for NFS).
    version: u32,
    file_acl: u32,
    dir_acl: u32,
    /// Block address of fragment.
    frag_addr:u32,
    frag_num: u32,
    reserved1: [u16; 5],
}
