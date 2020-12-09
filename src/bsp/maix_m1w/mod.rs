use core::fmt::Write;

use k210_hal::fpioa;
use k210_hal::pac::Peripherals;
use k210_hal::prelude::*;
use k210_hal::serial::Serial;
use k210_hal::stdout::Stdout;

use riscv::register::mhartid;

use crate::println;
use crate::sync::Spinlock;

#[no_mangle]
pub fn DefaultHandler() -> ! {
    use riscv::register::mcause::{Interrupt, Trap};
    let cause = riscv::register::mcause::read().cause();
    println!("Interrupt : {:?}", cause);

    match cause {
        Trap::Interrupt(int) => match int {
            Interrupt::UserSoft => {
                panic!("UserSoft Interrupt")
            }
            Interrupt::SupervisorSoft => {
                panic!("SupervisorSoft Interrupt")
            }
            Interrupt::MachineSoft => {
                panic!("MachineSoft Interrupt")
            }
            Interrupt::UserTimer => {
                panic!("UserTimer Interrupt")
            }
            Interrupt::SupervisorTimer => {
                panic!("SupervisorTimer Interrupt")
            }
            Interrupt::MachineTimer => {
                panic!("MachineTimer Interrupt")
            }
            Interrupt::UserExternal => {
                panic!("UserExternal Interrupt")
            }
            Interrupt::SupervisorExternal => {
                panic!("SupervisorExternal Interrupt")
            }
            Interrupt::MachineExternal => {
                panic!("MachineExternal Interrupt")
            }
            Interrupt::Unknown => {
                panic!("Unknown Interrupt")
            }
        },
        _ => panic!("Unhandled Exception in the Interrupt Handler!"),
    }
}

#[no_mangle]
pub fn ExceptionHandler(trap_frame: &mut riscv_rt::TrapFrame) {
    use riscv::register::mcause::{Exception, Trap};
    let cause = riscv::register::mcause::read().cause();
    println!("Exception : {:?}", cause);

    match cause {
        Trap::Exception(exc) => match exc {
            Exception::InstructionMisaligned => {
                panic!("InstructionMisaligned Exception")
            }
            Exception::InstructionFault => {
                panic!("InstructionFault Exception")
            }
            Exception::IllegalInstruction => {
                panic!("IllegalInstruction Exception")
            }
            Exception::Breakpoint => {
                panic!("Breakpoint Exception")
            }
            Exception::LoadMisaligned => {
                panic!("LoadMisaligned Exception")
            }
            Exception::LoadFault => {
                panic!("LoadFault Exception")
            }
            Exception::StoreMisaligned => {
                panic!("StoreMisaligned Exception")
            }
            Exception::StoreFault => {
                panic!("StoreFault Exception")
            }
            Exception::UserEnvCall => {
                panic!("UserEnvCall Exception")
            }
            Exception::SupervisorEnvCall => {
                panic!("SupervisorEnvCall Exception")
            }
            Exception::MachineEnvCall => {
                panic!("MachineEnvCall Exception")
            }
            Exception::InstructionPageFault => {
                panic!("InstructionPageFault Exception")
            }
            Exception::LoadPageFault => {
                panic!("LoadPageFault Exception")
            }
            Exception::StorePageFault => {
                panic!("StorePageFault Exception")
            }
            Exception::Unknown => {
                panic!("Unknown Exception")
            }
        },
        _ => panic!("Unhandled Interrupt in the Exception Handler!"),
    }
    loop {
        continue;
    }
}

pub fn init() {
    let p = Peripherals::take().unwrap();

    let mut sysctl = p.SYSCTL.constrain();
    // Prepare pins for UARTHS
    let fpioa = p.FPIOA.split(&mut sysctl.apb0);
    let _io5 = fpioa.io5.into_function(fpioa::UARTHS_TX);

    // Configure clocks (TODO)
    let clocks = k210_hal::clock::Clocks::new();

    // Configure UART
    let serial = p.UARTHS.configure(115_200.bps(), &clocks);
    let (mut tx, _) = serial.split();

    // todo: new stdout design (simple Write impl?)
    let mut stdout = Stdout(&mut tx);

    loop {
        writeln!(stdout, "Test").unwrap();
    }
}

pub fn critical_section<F, R>(f: F) -> R
where
    F: FnOnce(&riscv::interrupt::CriticalSection) -> R,
{
    riscv::interrupt::free(|cs| f(cs))
}
