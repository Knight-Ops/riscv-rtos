use core::sync::atomic::{AtomicBool, Ordering};
use lock_api::{GuardSend, RawMutex, RawRwLock, RwLock};

pub struct RawSpinlock(AtomicBool);

unsafe impl RawMutex for RawSpinlock {
    const INIT: RawSpinlock = RawSpinlock(AtomicBool::new(false));

    type GuardMarker = GuardSend;

    #[inline(never)]
    fn lock(&self) {
        while !self.try_lock() {}
    }

    #[inline(never)]
    fn try_lock(&self) -> bool {
        if !self.0.load(Ordering::SeqCst) {
            self.0.store(true, Ordering::SeqCst);
            true
        } else {
            false
        }
    }

    #[inline(never)]
    unsafe fn unlock(&self) {
        self.0.store(false, Ordering::SeqCst);
    }
}

pub type Spinlock<T> = lock_api::Mutex<RawSpinlock, T>;
pub type SpinlockGuard<'a, T> = lock_api::MutexGuard<'a, RawSpinlock, T>;


pub struct RawRwSpinlock {
    exclusive: AtomicBool, 
    shared: AtomicBool
}

unsafe impl RawRwLock for RawRwSpinlock {
    const INIT: RawRwSpinlock = RawRwSpinlock {exclusive: AtomicBool::new(false), shared: AtomicBool::new(false)};

    type GuardMarker = GuardSend;

    fn lock_shared(&self) {
        while !self.try_lock_shared() {}
    }

    fn try_lock_shared(&self) -> bool {
        if !self.exclusive.load(Ordering::SeqCst) {
            self.shared.store(true, Ordering::SeqCst);
            true
        } else {
            false
        }
    }

    unsafe fn unlock_shared(&self) {
        self.shared.store(false, Ordering::SeqCst);
    }

    fn lock_exclusive(&self) {
        while !self.try_lock_exclusive() {}
    }

    fn try_lock_exclusive(&self) -> bool {
        if !self.exclusive.load(Ordering::SeqCst) && !self.shared.load(Ordering::SeqCst) {
            self.exclusive.store(true, Ordering::SeqCst);
            true
        } else {
            false
        }
    }

    unsafe fn unlock_exclusive(&self) {
        self.exclusive.store(false, Ordering::SeqCst);
    }
}

pub type RwSpinlock<T> = lock_api::RwLock<RawRwSpinlock, T>;
pub type SpinlockReadGuard<'a, T> = lock_api::RwLockReadGuard<'a, RawRwSpinlock, T>;
pub type SpinlockWriteGuard<'a, T> = lock_api::RwLockWriteGuard<'a, RawRwSpinlock, T>;