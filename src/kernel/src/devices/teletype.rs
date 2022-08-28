use alloc::collections::vec_deque::VecDeque;
use crate::devices::{CharDeviceSwitch, DeviceError};

bitflags::bitflags! {
    pub struct TeletypeInputFlags: u64 {
        /// Ignore break conditions.
        const IGNORE_BREAK_COND = 1 << 0;
        /// Invoke an interrupt signal upon break.
        const BREAK_INTERRUPT = 1 << 1;
        /// Ignore characters with parity errors.
        const IGNORE_PARITY_ERRORS = 1 << 2;
        /// Mark parity and framing errors.
        const MARK_PARITY_FRAMING_ERRORS = 1 << 3;
        /// Enable input parity check.
        const INPUT_PARITY_CHECK = 1 << 4;
        /// Strip 8th bit off of characters.
        const STRIP_8TH_BIT = 1 << 5;
        /// Map new-line characters to carriage-return characters.
        const MAP_NL_CR = 1 << 6;
        /// Ignore carriage-return characters.
        const IGNORE_CARRIAGE_RETURN = 1 << 7;
        /// Conver to lower case.
        const CONVERT_LOWER = 1 << 8;
        /// Enable start-stop output control.
        const START_STOP_OUTPUT = 1 << 9;
        /// Any character will restart after a stop.
        const START_STOP_ANY = 1 << 10;
        /// Enable start-stop input control.
        const START_STOP_INPUT = 1 << 11;
        /// Ring bell when input queue is full.
        const QUEUE_FULL_BELL = 1 << 12;
    }
}

bitflags::bitflags! {
    pub struct TeletypeOutputFlags: usize {
        /// Post-process output.
        const POST_PROCESS_OUTPUT = 1 << 0;
        /// Map new-line characters to a carriage-return character followed by a new-line
        /// character.
        const MAP_NL_CRNL = 1 << 1;
        /// Map carriage-return characters to new-line characters.
        const MAP_CR_NL = 1 << 2;
    }
}

/// Control characters.
pub struct TeletypeControlChars {
}

/// Terminal control structure (also known as termios).
pub struct TeletypeControl {
    /// Input-mode flags.
    input_flags: TeletypeInputFlags,
    /// Output-mode flags.
    output_flags: TeletypeOutputFlags,
    /// Control-mode flags.
    control_flags: TeletypeControlFlags,
    /// Local-mode flags.
    local_flags: TeletypeLocalFlags,
    /// Line dicipline.
    line_dicipline: u8,
    /// Control characters.
    control_chars: TeletypeControlChars,
}

/// Teletypes abstract over character devices, just like a buffered-device would abstract over a
/// block device.
pub struct Teletype {
    /// Internal character device.
    device: dyn CharDeviceSwitch,
    /// Output queue.
    output_queue: VecDeque<u8>,
    /// Canonical queue. This is when terminal input is processed in lines terminated by \n. No
    /// input can be read until the entire line has been read by the user.
    canonical_queue: VecDeque<u8>,
    /// Raw input quque.
    input_queue: VecDeque<u8>,
    /// Control data.
    control: TeletypeControl,
}

impl CharDeviceSwitch for Teletype {
    fn put_char(&mut self, byte: u8) -> Result<(), DeviceError> {

    }

    fn get_char(&mut self, byte: u8) -> Result<(), DeviceError> {
        
    }
}
