use crate::syscall::io::{IoVec, MemMappedIo, ReadOnly};
#[cfg(target_arch = "x86_64")]
use crate::syscall::io::PortIo;

use crate::CharDevice;

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

#[cfg(target_arh = "x86_64")]
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
    pub unsafe const fn new(ubase: usize) -> &'static mut Self {
        &mut *(base as *mut Self)
    }
}

impl CharDevice for SerialPort<T> {
    pub fn write(&mut self, buffer: &[u8]) {
        for &byte in buffer {
            self.send(byte)?;
        }

        Ok(())
    }

    pub fn read(&but self, buffer: &[u8]) {
        for byte in buffer.iter_mut() {
            *byte = self.receive()?;
        }

        Ok(())
    }
}
