//! Stdout based on the UART hooked up to FTDI or J-Link
use core::fmt;
use e310x_hal::{
    clock::Clocks,
    e310x::UART0,
    gpio::gpio0::{Pin16, Pin17},
    prelude::*,
    serial::{Rx, Serial, Tx},
    time::Bps,
};
use nb::block;

pub struct SerialWrapper {
    tx: Tx<UART0>,
    rx: Rx<UART0>,
    chars_written: usize,
    chars_read: usize,
}

impl SerialWrapper {
    pub fn new(tx: Tx<UART0>, rx: Rx<UART0>) -> Self {
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
                let res = block!(self.tx.write('\r' as u8));

                if res.is_err() {
                    return Err(::core::fmt::Error);
                }
            }

            self.chars_written += 1;
            let res = block!(self.tx.write(*byte));

            if res.is_err() {
                return Err(::core::fmt::Error);
            }
        }
        Ok(())
    }
}

impl crate::console::interface::Write for SerialWrapper {
    fn write_char(&mut self, c: char) {
        self.chars_written += 1;
        block!(self.tx.write(c as u8));
    }

    fn write_fmt(&mut self, args: fmt::Arguments) -> fmt::Result {
        core::fmt::Write::write_fmt(self, args)
    }

    fn flush(&mut self) {
        self.tx.flush();
    }
}

impl crate::console::interface::Read for SerialWrapper {
    /// Read a single character.
    fn read_char(&mut self) -> char {
        self.chars_read += 1;
        block!(self.rx.read()).expect("Error while reading char") as char
    }

    /// Clear RX buffers, if any.
    fn clear(&mut self) {}
}

impl crate::console::interface::Statistics for SerialWrapper {
    fn chars_read(&mut self) -> usize {
        self.chars_read
    }

    fn chars_written(&mut self) -> usize {
        self.chars_written
    }
}

/// Configures stdout
pub fn configure<X, Y>(uart: UART0, tx: Pin17<X>, rx: Pin16<Y>, baud_rate: Bps, clocks: Clocks) {
    let tx = tx.into_iof0();
    let rx = rx.into_iof0();
    let serial = Serial::new(uart, (tx, rx), baud_rate, clocks);
    let (tx, rx) = serial.split();

    let mut console = SerialWrapper::new(tx, rx);

    super::critical_section(|_| super::CONSOLE.lock().borrow_mut().replace(console));
}
