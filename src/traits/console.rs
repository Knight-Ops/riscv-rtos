/// Console interfaces.
pub mod interface {
    use core::fmt;

    /// Console write functions.
    pub trait Write {
        /// Write a single character.
        fn write_char(&mut self, c: char);

        /// Write a Rust format string.
        fn write_fmt(&mut self, args: fmt::Arguments) -> fmt::Result;

        /// Block execution until the last character has been physically put on the TX wire
        /// (draining TX buffers/FIFOs, if any).
        fn flush(&mut self);
    }

    /// Console read functions.
    pub trait Read {
        /// Read a single character.
        fn read_char(&mut self) -> char {
            ' '
        }

        /// Clear RX buffers, if any.
        fn clear(&mut self);
    }

    /// Console statistics.
    pub trait Statistics {
        /// Return the number of characters written.
        fn chars_written(&self) -> usize {
            0
        }

        /// Return the number of characters read.
        fn chars_read(&self) -> usize {
            0
        }
    }

    /// Trait alias for a full-fledged console.
    pub trait All = Write + Read + Statistics;

}
