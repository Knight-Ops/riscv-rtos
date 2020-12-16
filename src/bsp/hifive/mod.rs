use core::cell::RefCell;
use riscv;

use hifive1::hal::prelude::*;
use hifive1::hal::DeviceResources;
use hifive1::pin;

use e310x_hal::{
    clock::Clocks,
    e310x::UART0,
    gpio::gpio0::{Pin16, Pin17},
    prelude::*,
    serial::{Rx, Serial, Tx},
    time::Bps,
};

use hifive1::hal::core::clint::Clint;

pub mod console;
pub mod time;

use crate::{sync::{RwSpinlock, Spinlock}};
use crate::{mut_spinlock, println, read_spinlock, spinlock, write_spinlock};
use crate::traits::board::BoardSupportPackage;

// Global Instances
pub static CONSOLE: Spinlock<RefCell<Option<console::SerialWrapper>>> =
    Spinlock::new(RefCell::new(None));

pub static CORELOCAL_INTERRUPT: Spinlock<RefCell<Option<Clint>>> =
    Spinlock::new(RefCell::new(None));
// End Global Instances


#[no_mangle]
pub fn DefaultHandler() {
    use riscv::register::mcause::{Interrupt, Trap};
    let cause = riscv::register::mcause::read().cause();
    // println!("Interrupt : {:?}", cause);

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
                // let next_interrupt = CORELOCAL_INTERRUPT.lock().borrow().as_ref().unwrap().mtime.mtime() + 0xA_0000;
                // CORELOCAL_INTERRUPT.lock().borrow_mut().as_mut().unwrap().mtimecmp.set_mtimecmp(next_interrupt);
                // println!("Tick");
                // return;
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

    loop {}
}

#[no_mangle]
pub fn ExceptionHandler(trap_frame: &mut riscv_rt::TrapFrame) {
    use riscv::register::mcause::{Exception, Trap};
    let cause = riscv::register::mcause::read().cause();
    // println!("Exception : {:?}", cause);

    match cause {
        Trap::Exception(exc) => {
            match exc {
                Exception::InstructionMisaligned => {
                    panic!("InstructionMisaligned Exception")
                }
                Exception::InstructionFault => {
                    panic!("InstructionFault Exception")
                }
                Exception::IllegalInstruction => {
                    let mtval = riscv::register::mtval::read();
                    match mtval {
                        // rdtime a0
                        0xc0102573 => {
                            trap_frame.a0 =
                                spinlock!(CORELOCAL_INTERRUPT).mtime.mtime_lo() as usize;
                            riscv::register::mepc::write(riscv::register::mepc::read() + 4);
                            return;
                        }
                        // rdtimeh a0
                        0xc8102573 => {
                            trap_frame.a0 =
                                spinlock!(CORELOCAL_INTERRUPT).mtime.mtime_hi() as usize;
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
            }
        }
        _ => panic!("Unhandled Interrupt in the Exception Handler!"),
    }
    loop {
        continue;
    }
}

pub struct Hifive1RevB;

impl Hifive1RevB {
    pub fn get_clint() -> &'static Spinlock<RefCell<Option<Clint>>> {
        &CORELOCAL_INTERRUPT
    }
}

impl BoardSupportPackage for Hifive1RevB {
    fn early_init(){
        let dr = DeviceResources::take().unwrap();
        let cp = dr.core_peripherals;
        let p = dr.peripherals;
        let pins = dr.pins;
    
        // Configure clocks
        let clocks = hifive1::clock::configure(p.PRCI, p.AONCLK, 320.mhz().into());
    
        // Populate the CLINT Peripheral behind the Mutex
        CORELOCAL_INTERRUPT.lock().borrow_mut().replace(cp.clint);

        // Configure UART for stdout
        console::configure(
            p.UART0,
            pin!(pins, uart0_tx),
            pin!(pins, uart0_rx),
            115_200.bps(),
            clocks,
        );
    
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