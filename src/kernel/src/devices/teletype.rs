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
        /// No carriage return at column 0.
        const NO_CR_COL0 = 1 << 3;
        /// New-line characters perform the function of carriage-return characters.
        const NL_AS_CR = 1 << 4;
        /// Fill characters are DELETEs.
        const FILL_DELETE = 1 << 5;
        /// Use fill characters for delay.
        const FILL_DELAY = 1 << 6;
    }
}

bitflags::bitflags! {
    pub struct TeletypeControlFlags: usize {
        const HANG_UP = 1;
        const BAUD_50 = 1;
        const BAUD_75 = 2;
        const BAUD_110 = 3;
        const BAUD_134 = 4;
        const BAUD_150 = 5;
        const BAUD_200 = 6;
        const BAUD_300 = 7;
        const BAUD_600 = 8;
        const BAUD_1200 = 9;
        const BAUD_1800 = 10;
        const BAUD_2400 = 11;
        const BAUD_2800 = 12;
        const BAUD_9600 = 13;
        const BAUD_19200 = 14;
        const BAUD_38400 = 15;
    }
}

bitflags::bitflags! {
    pub struct TeletypeLocalFlags: usize {
        /// Enable echo.
        const LOCAL = 1 << 0;
        /// Echo erase-character as error-correcting backspace.
        const ECHO_ERASE = 1 << 1;
        /// Echo kill-characters.
        const ECHO_KILL = 1 << 2;
        /// Echo new-line characters.
        const ECHO_NL = 1 << 3;
        /// Canonical input (erase and kill processing).
        const CANONICAL = 1 << 4;
        /// Enable extended input-processing.
        const EXTENDED_INPUT = 1 << 5;
        /// Enable signals.
        const SIGNALS = 1 << 6;
        /// Disable flush after interrupt or quit signal.
        const NO_FLUSH = 1 << 7;
        /// Send SIGTTOU for background output.
        const OUTPUT_STOP = 1 << 8;
    }
}

/// Control characters.
pub struct TeletypeControlChars {
    interrupt: u8,
    quit: u8,
    erase: u8,
    kill: u8,
    end_of_file: u8,
    /// Time-out value (measured in 1/10ths of a second).
    timeout: u8,
    /// Minimum number of bytes that can be read at once.
    min_bytes: u8,
    swtc: u8,
    start: u8,
    stop: u8,
    suspend: u8,
    end_of_line: u8,
    reprint_line: u8,
    discard: u8,
    word_erase: u8,
    literal_next: u8,
    end_of_line2: u8,
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
