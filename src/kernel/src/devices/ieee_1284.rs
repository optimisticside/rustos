#[cfg(target_arch = "x86_64")]
use crate::io::PortIo;
use crate::io::{IoVec, MemMappedIo, ReadOnly};

use crate::devices::{CharDeviceSwitch, DeviceError};

bitflags::bitflags! {
    struct StatusFlags: u8 {
        const IRQ = 1 << 2;
        /// There is no error.
        const NO_ERROR = 1 << 3;
        const SELECT_IN = 1 << 4;
        const PAPER_OUT = 1 << 5;
        const ACK = 1 << 6;
        /// Printer is currently busy.
        const BUSY = 1 << 7;
    }
}

bitflags::bitflags! {
    struct ControlFlags: u8 {
        /// Toggled to assert wait cycle.
        const NO_STROBE = 1 << 0;
        /// Automatically insert a line-feed.
        const NO_AUTO_LF = 1 << 1;
        /// Reset printer.
        const RESET = 1 << 2;
        /// Select printer.
        const NO_SELECT_OUT = 1 << 3;
        /// Toggle IRQ through the acknowledgement line.
        const IRQ_ACK = 1 << 4;
        /// Toggles bi-directional port. Allows you to input 8-bits using the data port. Only
        /// possible if card supports this.
        const BI_DIRECTIONAL = 1 << 5;
    }
}

/// One allocated per parallel port.
#[repr(C, packed)]
pub struct ParallelPort<T: IoVec> {
    /// Data register. Read to recieve and write to send.
    data: T,
    /// Status register.
    status: ReadOnly<T>,
    /// Control register.
    control: T,
}

#[cfg(target_arch = "x86_64")]
impl ParallelPort<PortIo<u8>> {
    pub const fn new(base: u16) -> Self {
        Self {
            data: PortIo::new(base),
            status: ReadOnly::new(PortIo::new(base + 1)),
            control: PortIo::new(base + 2),
        }
    }
}

impl ParallelPort<MemMappedIo<u32>> {
    pub const unsafe fn new(base: usize) -> &'static mut Self {
        &mut *(base as *mut Self)
    }
}

impl<T: IoVec> ParallelPort<T>
where
    T::Value: From<u8> + TryInto<u8>,
{
    /// Retrieve the value of the status register.
    fn status(&self) -> StatusFlags {
        StatusFlags::from_bits_truncate(
            (self.status.read() & 0xFF.into())
                .try_into()
                .unwrap_or(0),
        )
    }

    /// Retrieve the value of the control register.
    fn control(&self) -> ControlFlags {
        ControlFlags::from_bits_truncate(
            (self.status.read & 0xff.into())
                .try_into()
                .unwrap_or(0)
        )
    }
}

impl<T: IoVec> CharDeviceSwitch for ParallelPort<T>
where
    T::Value: From<u8> + TryInto<u8>,
{
    /// Read a byte from the parallel port.
    fn get_char(&self) -> Result<u8, DeviceError> {
        if self.line_status().contains(LineStatusFlags::INPUT_FULL) {
            return Ok((self.data.read() & 0xFF.into()).try_into().unwrap_or(0));
        }

        // TODO: This should be an error here, but I have yet to implement devices::DeviceError.
        Ok(0)
    }

    /// Write a character to the port. Note that there is no abstraction over new-lines like there
    /// is on UNIX, where you can substitute `\r\n` for `\n`.
    fn put_char(&mut self, byte: u8) -> Result<(), DeviceError> {
        while !self.status().contains(LineStatusFlags::OUTPUT_EMPTY) {}
        self.data.write(byte.into());
        Ok(())
    }
}
