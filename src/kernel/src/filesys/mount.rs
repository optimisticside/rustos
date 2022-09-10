use crate::filesys::Vnode;

/// Structure per mounted file-system. Each mounted file-system has an array of operations in an
/// instance record.
pub struct Mount {
    /// Root V-node.
    pub root: Vnode,
    /// V-node that we mounted on.
    pub parent: Option<Vnode>,
}
