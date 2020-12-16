//! Stdout based on the UART hooked up to FTDI or J-Link
use core::fmt;
use k210_hal::fpioa;
use k210_hal::pac::{Peripherals, UARTHS};
use k210_hal::prelude::*;
use k210_hal::serial::{Serial, Tx, Rx};
use k210_hal::stdout::Stdout;
use nb::block;

use crate::traits::board::BoardSupportPackage;

pub struct SerialWrapper {
    tx: Tx<UARTHS>,
    rx: Rx<UARTHS>,
    chars_written: usize,
    chars_read: usize,
}

impl SerialWrapper {
    pub fn new(tx: Tx<UARTHS>, rx: Rx<UARTHS>) -> Self {
        SerialWrapper {
            tx,
            rx,
            chars_written: 0,
            chars_read: 0,
        }
    }
}

impl core::fmt::Write for SerialWrapper {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.as_bytes() {
            if *byte == '\n' as u8 {
                self.chars_written += 1;
                let res = block!(self.tx.try_write('\r' as u8));

                if res.is_err() {
                    return Err(::core::fmt::Error);
                }
            }

            self.chars_written += 1;
            let res = block!(self.tx.try_write(*byte));

            if res.is_err() {
                return Err(::core::fmt::Error);
            }
        }
        Ok(())
    }
}

impl crate::traits::console::interface::Write for SerialWrapper {
    fn write_char(&mut self, c: char) {
        self.chars_written += 1;
        block!(self.tx.try_write(c as u8));
    }

    fn write_fmt(&mut self, args: fmt::Arguments) -> fmt::Result {
        core::fmt::Write::write_fmt(self, args)
    }

    fn flush(&mut self) {
        self.tx.try_flush();
    }
}

impl crate::traits::console::interface::Read for SerialWrapper {
    /// Read a single character.
    fn read_char(&mut self) -> char {
        self.chars_read += 1;
        block!(self.rx.try_read()).expect("Error while reading char") as char
    }

    /// Clear RX buffers, if any.
    fn clear(&mut self) {}
}

impl crate::traits::console::interface::Statistics for SerialWrapper {
    fn chars_read(&self) -> usize {
        self.chars_read
    }

    fn chars_written(&self) -> usize {
        self.chars_written
    }
}