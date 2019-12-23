use core::sync::atomic::*;

use syscalls::*;

use crate::{time_t, timespec, tv_nsec_t};
use crate::constants::*;
use crate::time::Instant;

pub struct ThreadParker {
    futex: AtomicI32,
}

pub trait UnparkHandleT {
    /// Wakes up the parked thread. This should be called after the queue lock is
    /// released to avoid blocking the queue for too long.
    ///
    /// This method is unsafe for the same reason as the unsafe methods in
    /// `ThreadParkerT`.
    unsafe fn unpark(self);
}


pub trait ThreadParkerT {
    type UnparkHandle: UnparkHandleT;

    const IS_CHEAP_TO_CONSTRUCT: bool;

    fn new() -> Self;

    /// Prepares the parker. This should be called before adding it to the queue.
    unsafe fn prepare_park(&self);

    /// Checks if the park timed out. This should be called while holding the
    /// queue lock after park_until has returned false.
    unsafe fn timed_out(&self) -> bool;

    /// Parks the thread until it is unparked. This should be called after it has
    /// been added to the queue, after unlocking the queue.
    unsafe fn park(&self);

    /// Parks the thread until it is unparked or the timeout is reached. This
    /// should be called after it has been added to the queue, after unlocking
    /// the queue. Returns true if we were unparked and false if we timed out.
    unsafe fn park_until(&self, timeout: Instant) -> bool;

    /// Locks the parker to prevent the target thread from exiting. This is
    /// necessary to ensure that thread-local ThreadData objects remain valid.
    /// This should be called while holding the queue lock.
    unsafe fn unpark_lock(&self) -> Self::UnparkHandle;
}

impl ThreadParkerT for ThreadParker {
    type UnparkHandle = UnparkHandle;

    const IS_CHEAP_TO_CONSTRUCT: bool = true;

    #[inline]
    fn new() -> ThreadParker {
        ThreadParker {
            futex: AtomicI32::new(0),
        }
    }

    #[inline]
    unsafe fn prepare_park(&self) {
        self.futex.store(1, Ordering::Relaxed);
    }

    #[inline]
    unsafe fn timed_out(&self) -> bool {
        self.futex.load(Ordering::Relaxed) != 0
    }

    #[inline]
    unsafe fn park(&self) {
        while self.futex.load(Ordering::Acquire) != 0 {
            self.futex_wait(None);
        }
    }

    #[inline]
    unsafe fn park_until(&self, timeout: Instant) -> bool {
        while self.futex.load(Ordering::Acquire) != 0 {
            let now = Instant::now();
            if timeout <= now {
                return false;
            }
            let diff = timeout - now;
            if diff.as_secs() as time_t as u64 != diff.as_secs() {
                // Timeout overflowed, just sleep indefinitely
                self.park();
                return true;
            }
            let ts = timespec {
                tv_sec: diff.as_secs() as time_t,
                tv_nsec: diff.subsec_nanos() as tv_nsec_t,
            };
            self.futex_wait(Some(ts));
        }
        true
    }

    // Locks the parker to prevent the target thread from exiting. This is
    // necessary to ensure that thread-local ThreadData objects remain valid.
    // This should be called while holding the queue lock.
    #[inline]
    unsafe fn unpark_lock(&self) -> UnparkHandle {
        // We don't need to lock anything, just clear the state
        self.futex.store(0, Ordering::Release);

        UnparkHandle { futex: &self.futex }
    }
}

impl ThreadParker {
    #[inline]
    fn futex_wait(&self, ts: Option<timespec>) {
        let ts_ptr = ts
            .as_ref()
            .map(|ts_ref| ts_ref as *const _)
            .unwrap_or(0 as _);
        unsafe {
            match syscall4(
                SYS_futex,
                &self.futex as *const _ as _,
                (FUTEX_WAIT | FUTEX_PRIVATE_FLAG) as _,
                1,
                ts_ptr as _,
            ) { _ => () };
        };
    }
}

pub struct UnparkHandle {
    futex: *const AtomicI32,
}

impl UnparkHandleT for UnparkHandle {
    #[inline]
    unsafe fn unpark(self) {
        // The thread data may have been freed at this point, but it doesn't
        // matter since the syscall will just return EFAULT in that case.
        syscall3(
            SYS_futex,
            self.futex as _,
            (FUTEX_WAKE | FUTEX_PRIVATE_FLAG) as _,
            1,
        ).unwrap(); // TODO : WRAP THIS
    }
}

#[inline]
pub fn thread_yield() {
    unsafe {
        crate::utils::cpu::cpu_relax();
        crate::utils::cpu::thread_yield();
    }
}