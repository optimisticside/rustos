/// Representation of an error as the result of any operation done on the system. These includes
/// device errors, file-system errors, and other types of errors.
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Error { 
    /// File-system errors.
    NotSupported,
    EntryExists,
    EntryNotFound,
    Busy,
    NotDirectory,
    IsPipe,
    IsDirectory,
    Interrupted,
    TooSmall,
    InvalidPath,
    NotSocket,
    ConnectionRefused,
    NotConnected,
    WouldBlock,
}
