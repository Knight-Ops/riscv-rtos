use crate::{print, println};

#[no_mangle]
extern "C" fn eh_personality() {}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("Panic : ");
    if let Some(p) = info.location() {
        println!(
            "\tLine :  {}\n\tFile : {}\n\tReason : {}",
            p.line(),
            p.file(),
            info.message().unwrap()
        );
    } else {
        println!("No information available.");
    }
    _abort()
}

#[no_mangle]
extern "C" fn _abort() -> ! {
    loop {
        unsafe { asm!("wfi") }
    }
}
