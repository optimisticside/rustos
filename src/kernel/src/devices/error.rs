/// Representation of an error as the result of an I/O operation on a device. Provided through all
/// I/O routines of devices (since they all return a `Result`).
#[derive(Debug, Clone)]
pub struct DeviceError;
