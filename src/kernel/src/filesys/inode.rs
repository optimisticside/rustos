/// Types of I-nodes.
enum InodeKind {
    Null,
    Directory,
    Block,
    Character,
    Link,
    Socket,
    Fifo,
    Bad,
}

/// An I-node is the focus of file activity on UNIX system. There is one allocated for every active
/// file, directory, mounted-file, and the file-system's root.
pub struct Inode {
    /// The type of the I-node.
    kind: InodeKind,
    /// Specific fields that are specific to the type of the I-node.
    
}
