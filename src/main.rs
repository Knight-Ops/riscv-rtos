#![feature(core_intrinsics)]
#![feature(trait_alias)]
#![feature(format_args_nl)]
#![feature(panic_info_message)]
#![feature(asm)]
#![no_std]
#![no_main]

mod traits;
mod bsp;
// mod driver;
mod panic;
mod sync;
mod page_allocator;

#[macro_use]
mod macros;

// #[macro_use]
// mod print;

#[cfg(any(feature = "bsp_hifive", feature = "bsp_maix_m1w"))]
use riscv_rt::entry;

use bsp::target_board as target_board;
use traits::board::BoardSupportPackage;
use traits::allocator::Allocator;
use traits::console::interface::All;

#[entry]
unsafe fn bootloader_entry() -> ! {
    target_board::early_init();

    if target_board::get_console().lock().borrow().is_none() {
        panic!("Early Initialization failed to populate console!");
    }

    println!("-------------------------------------------------------\n      Bootloader Early Initialization Completed!       \n-------------------------------------------------------");

    page_allocator::PageAllocator::init();

    // println!("Page allocator Initialized!");

    // println!("I have allocated {} pages @ {:X?}",1, page_allocator::PageAllocator::alloc(1));

    let printed_bytes = target_board::get_console().lock().borrow().as_ref().unwrap().chars_written();
    println!(
        "I have printed {} bytes",
        printed_bytes
    );

    // riscv::register::mie::set_mtimer();
    // riscv::register::mie::set_mext();
    // riscv::register::mie::set_msoft();
    riscv::interrupt::enable();

    loop {
        // let time = riscv::register::time::read();
        let time = spinlock!(target_board::get_clint()).mtime.read().bits();

        if time % 0x1_0000 == 0 {
            println!("Tick : {}", time);
        }
    }
}
