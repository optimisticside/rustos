/// Representation of an error as the result of an I/O operation on a device. Provided through all
/// I/O routines of devices (since they all return a `Result`).
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DeviceError;
