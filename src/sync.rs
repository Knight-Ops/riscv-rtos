use core::sync::atomic::{AtomicBool, Ordering};
use lock_api::{GuardSend, RawMutex};

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
