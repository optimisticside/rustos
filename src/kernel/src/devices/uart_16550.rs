use crate::io::{IoVec, MemMappedIo, ReadOnly};
#[cfg(target_arch = "x86_64")]
use crate::io::PortIo;

use crate::devices::CharDevice;

bitflags::bitflags! {
    struct LineStatusFlags: u8 {
        const INPUT_FULL = 1;
        // Bits 1-4: unknown
        const OUTPUT_FULL = 1 << 5;
        // Bits 6-8: unknown
    }
}

/// One allocated per serial port.
#[repr(C, packed)]
pub struct SerialPort<T: IoVec> {
    /// Data register. Read to recieve and write to send.
    data: T,
    /// Interrupt enable.
    int_enable: T,
    /// FIFO control register.
    fifo_control: T,
    /// Line control register.
    line_control: T,
    /// Modem control register.
    modem_control: T,
    /// Line status register.
    line_status: ReadOnly<T>,
    /// Modem status register.
    modem_status: ReadOnly<T>,
}

#[cfg(target_arch = "x86_64")]
impl SerialPort<PortIo<u8>> {
    pub const fn new(base: u16) -> Self {
        Self {
            data: PortIo::new(base),
            int_enable: PortIo::new(base + 1),
            fifo_control: PortIo::new(base + 2),
            line_control: PortIo::new(base + 3),
            modem_control: PortIo::new(base + 4),
            line_status: PortIo::new(base + 5),
            modem_status: PortIo::new(base + 6),
        }
    }
}

impl SerialPort<MemMappedIo<u32>> {
    pub const unsafe fn new(base: usize) -> &'static mut Self {
        &mut *(base as *mut Self)
    }
}

impl<T: IoVec> SerialPort<T> {
    /// Initialize the serial port so that it can start receiving data and writing it.
    pub fn init(&mut self) {
        self.int_enable.write(0x00.into());
        self.line_control.write(0x80.into());
        self.data.write(0x01.into());
        self.int_enable.write(0x00.into());
        self.line_control.write(0x03.into());
        self.fifo_control.write(0xC7.into());
        self.modem_contrl.write(0x0B.into());
        self.int_enable.write(0x01.into());
    }

    /// Retrieve the value of the line-status register.
    fn line_status(&self) -> LineStatusFlags {
        LineStatusFlags::from_bits_truncate(
            (self.line_status.read() & 0xFF.into())
                .try_into()
                .unwrap_or(0)
        )
    }
}

impl<T: IoVec> CharDevice for  SerialPort<T> where T::Value: From<u8> + TryInto<u8> {
    /// Read a byte from the serial port.
    pub fn read_char(&self) -> u8 {
        if self.line_status().contains(LineStatusFlags::INPUT_FULL) {
            Some(
                (self.data.read() & 0xFF.into())
                    .try_into()
                    .unwrap_or(0),
            )
        }

        None
    }

    /// Write a character to the port. Note that there is no abstraction over new-lines like there
    /// is on UNIX, where you can substitute `\r\n` for `\n`.
    pub fn put_char(&mut self, byte: u8) {
        while !self.line_status().contains(LineStatusFlags::OUTPUT_EMPTY) {}
        self.data.write(byte.into());
    }
}
