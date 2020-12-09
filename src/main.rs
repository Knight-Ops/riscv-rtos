#![feature(core_intrinsics)]
#![feature(trait_alias)]
#![feature(format_args_nl)]
#![feature(panic_info_message)]
#![feature(asm)]
#![no_std]
#![no_main]

mod bsp;
mod console;
mod driver;
mod panic;
mod sync;
mod time;

#[macro_use]
mod print;

#[cfg(any(feature = "bsp_hifive", feature = "bsp_maix_m1w"))]
use riscv_rt::entry;

use console::interface::All;

#[entry]
unsafe fn bootloader_entry() -> ! {
    bsp::init();

    println!("In bootloader_entry!");

    println!(
        "I have printed {} bytes",
        bsp::critical_section(|_| bsp::CONSOLE
            .lock()
            .borrow_mut()
            .as_mut()
            .unwrap()
            .chars_written())
    );

    riscv::register::mie::set_mtimer();
    riscv::interrupt::enable();

    loop {
        let instret = riscv::register::minstret::read();

        if instret % 0xA000 == 0 {
            println!(
                "Instructions Retired : {}",
                instret
            );
        }
    }
}
