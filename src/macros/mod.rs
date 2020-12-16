pub mod print;
pub use print::*;

#[macro_export]
macro_rules! spinlock {
    ($i:expr) => {
        $i.lock().borrow().as_ref().unwrap()
    };
}

#[macro_export]
macro_rules! mut_spinlock {
    ($i:expr) => {
        $i.lock().borrow_mut().as_mut().unwrap()
    };
}

#[macro_export]
macro_rules! read_spinlock {
    ($i:expr) => {
        $i.read().borrow().as_ref().unwrap()
    };
}

#[macro_export]
macro_rules! write_spinlock {
    ($i:expr) => {
        $i.write().borrow_mut().as_mut().unwrap()
    };
}
