/// The super-block describes the file system on the disk. It gives us all the information we need
/// to read and write to the file system, such as where to find i-nodes and zones (blocks).
#[repr(C)]
pub struct SuperBlock {
	pub inode_count: u32,
	pub pad0: u16,
	pub imap_blocks: u16,
	pub zmap_blocks: u16,
	pub first_data_zone: u16,
	pub log_zone_size: u16,
	pub pad1: u16,
	pub max_size: u32,
	pub zones: u32,
	pub magic: u16,
	pub pad2: u16,
	pub block_size: u16,
	pub disk_version: u8
}

/// An I-node stores the meta-data of a file.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Inode {
    /// File permissions and type.
    pub mode: u16,
    /// Number of hard links. Inode is unnallocated when this reaches 0.
    pub link_count: u16,
    pub user_id: u16,
    pub group_id: u16,
    pub size: u16,
    /// Time of last access.
    pub access_time: u16,
    /// Time of last modification.
    pub modify_time: u16,
    /// Time of creation.
    pub create_time: u16,
    /// Points to location of blocks where the file's data is stored.
    pub zones: [u32; 10],
}

/// Note that I-nodes do not contain names. This is because more than one file can point to the
/// same I-node. These are called hard-lines (and are counted in Inode::link_count). A
/// directory-entry represents the association of a file with a name and an I-node on the disk.
#[repr(C)]
pub struct DirEntry {
    /// Block number of the associated I-node.
    pub inode: 32,
    /// Name of the file with a 60-character limit.
    pub name: [u8: 60],
}
