use crate::device::DeviceError;
use crate::filesys::FileSystemError;

/// Representation of an error as the result of any operation done on the system. These includes
/// device errors, file-system errors, and other types of errors.
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Error {
    /// File-system error.
    FileSystem(FileSystemError),
    /// Device error.
    DeviceError(DeviceError),
}
