use core::fmt::Write;
use core::cell::RefCell;
use riscv;

use k210_hal::fpioa;
use k210_hal::pac::{Peripherals, CLINT};
use k210_hal::prelude::*;
use k210_hal::serial::Serial;
use k210_hal::stdout::Stdout;

use riscv::register::mhartid;

use crate::{traits::board::BoardSupportPackage, println};
use crate::sync::Spinlock;
use crate::spinlock;

mod console;

// Global Instances
pub static CONSOLE: Spinlock<RefCell<Option<console::SerialWrapper>>> =
    Spinlock::new(RefCell::new(None));

pub static CORELOCAL_INTERRUPT: Spinlock<RefCell<Option<CLINT>>> =
    Spinlock::new(RefCell::new(None));
// End Global Instances

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
                let mtval = riscv::register::mtval::read();
                println!("MTVAL is : {:X}\nOccured @ {:X}", mtval, trap_frame.ra);
                match mtval {
                    // rdtime a0
                    0xc0102573 => {
                        trap_frame.a0 =
                            spinlock!(CORELOCAL_INTERRUPT).mtime.read().bits() as usize;
                        riscv::register::mepc::write(riscv::register::mepc::read() + 4);
                        return;
                    }
                    _ => {
                        panic!("IllegalInstruction Exception")
                    }
                }
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

pub struct MaixM1W;

impl MaixM1W {
    pub fn get_clint() -> &'static Spinlock<RefCell<Option<CLINT>>> {
        &CORELOCAL_INTERRUPT
    }
}

impl BoardSupportPackage for MaixM1W {
    fn early_init() {
        let p = Peripherals::take().unwrap();
    
        let mut sysctl = p.SYSCTL.constrain();
        // Prepare pins for UARTHS
        let fpioa = p.FPIOA.split(&mut sysctl.apb0);
        let _io5 = fpioa.io5.into_function(fpioa::UARTHS_TX);
    
        // Configure clocks (TODO)
        let clocks = k210_hal::clock::Clocks::new();

        CORELOCAL_INTERRUPT.lock().borrow_mut().replace(p.CLINT);
    
        // Configure UART
        let serial = p.UARTHS.configure(115_200.bps(), &clocks);
        let (mut tx, rx) = serial.split();

        let mut console = console::SerialWrapper::new(tx, rx);

        MaixM1W::critical_section(|_| CONSOLE.lock().borrow_mut().replace(console));
    
    }

    type ConsoleType = console::SerialWrapper;
    fn get_console() -> &'static Spinlock<RefCell<Option<Self::ConsoleType>>> {
        &CONSOLE
    }

    type CriticalSectionType = riscv::interrupt::CriticalSection;
    fn critical_section<F, R>(f: F) -> R
    where F: FnOnce(&Self::CriticalSectionType) -> R {
        riscv::interrupt::free(|cs| f(cs))
    }
}
