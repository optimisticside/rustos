use core::convert::From;
use crate::devices::DeviceError;

/// Representation of an error as the result of an file-operation on a file-system. Provided through
/// all I/O routines of file-systems (since they all return a `Result`).
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FileSystemError {
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

impl From<DeviceError> for FileSystemError {
    fn from(device_error: DeviceError) -> Self {
        match device_error {
            
        }
    }
}
