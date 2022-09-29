use alloc::vec::Vec;

/// Represents a socket address.
#[derive(Debug, Clone)]
pub enum SocketAddress<'a> {
    Unix(&SocketAddressUnix),
    Inet(&SocketAddressInet),
}
