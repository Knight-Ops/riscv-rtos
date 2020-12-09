#[macro_export]
macro_rules! spinlock {
    ($i:ident) => {
        $i.lock().borrow().as_ref().unwrap()
    };
}

#[macro_export]
macro_rules! mut_spinlock {
    ($i:ident) => {
        $i.lock().borrow_mut().as_mut().unwrap()
    };
}

#[macro_export]
macro_rules! read_spinlock {
    ($i:ident) => {
        $i.read().borrow().as_ref().unwrap()
    };
}

#[macro_export]
macro_rules! write_spinlock {
    ($i:ident) => {
        $i.write().borrow_mut().as_mut().unwrap()
    };
}