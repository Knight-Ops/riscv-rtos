use core::fmt::Write;

extern crate panic_halt;

use k210_hal::prelude::*;
use k210_hal::pac::Peripherals;
use k210_hal::serial::Serial;
use k210_hal::stdout::Stdout;
use k210_hal::fpioa;

use riscv::register::mhartid;

pub fn init() {
    let p = Peripherals::take().unwrap();

    let mut sysctl = p.SYSCTL.constrain();
    // Prepare pins for UARTHS
    let fpioa = p.FPIOA.split(&mut sysctl.apb0);
    let _io5 = fpioa.io5.into_function(fpioa::UARTHS_TX);

    // Configure clocks (TODO)
    let clocks = k210_hal::clock::Clocks::new();

    // Configure UART
    let serial = p.UARTHS.configure(
        115_200.bps(), 
        &clocks
    );
    let (mut tx, _) = serial.split();

    // todo: new stdout design (simple Write impl?)
    let mut stdout = Stdout(&mut tx);

    loop {
        writeln!(stdout, "Test").unwrap();
    }
}
