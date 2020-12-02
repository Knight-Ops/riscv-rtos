#![feature(core_intrinsics)]
#![feature(asm)]
#![no_std]
#![no_main]

mod bsp;
mod console;
mod driver;
mod synchronization;
mod time;

#[cfg(any(feature = "bsp_hifive", feature = "bsp_maix_m1w"))]
use riscv_rt::entry;

#[entry]
unsafe fn bootloader_entry() -> ! {
    bsp::init();

    loop {}
}
