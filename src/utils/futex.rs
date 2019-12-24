use core::cell::UnsafeCell;
use core::intrinsics::*;
use core::ops::*;
use core::sync::atomic::AtomicBool;
use core::sync::atomic::AtomicUsize;
use core::sync::atomic::Ordering;
use core::sync::atomic::Ordering::SeqCst;

use syscalls::*;

use crate::constants::*;

#[repr(C)]
struct RawFutex {
    locked: AtomicBool,
    waiter: AtomicUsize,
}

impl RawFutex {
    #[inline(always)]
    fn wait(&self) {
        for _ in 0..100 {
            if self.waiter.load(Ordering::SeqCst) == 0 && !self.locked.load(Ordering::SeqCst) {
                return;
            }
            unsafe {
                crate::thread_yield();
                core::sync::atomic::spin_loop_hint();
            }
        }
        self.waiter.fetch_add(1, SeqCst);
        unsafe {
            match syscall4(SYS_futex, &self.locked as *const AtomicBool as _, (FUTEX_WAIT | FUTEX_PRIVATE_FLAG) as _, 1, 0) {
                _ => ()
            }
        }
        self.waiter.fetch_sub(1, SeqCst);
    }

    #[inline(always)]
    fn lock(&self) {
        self.wait();
        while self.locked.compare_exchange(false, true, SeqCst, SeqCst).is_err() {
            unsafe {
                crate::thread_yield();
                core::sync::atomic::spin_loop_hint();
            }
        }
    }

    #[inline(always)]
    fn unlock(&self) {
        self.locked.store(false, SeqCst);
        unsafe {
            match syscall3(SYS_futex, &self.locked as *const AtomicBool as _, (FUTEX_WAKE | FUTEX_PRIVATE_FLAG) as _, 1) {
                _ => ()
            }
        }
    }
}

pub struct FutexGuard<'a, T> {
    ele: &'a mut T,
    __inner: &'a RawFutex,
}

impl<'a, T> Drop for FutexGuard<'a, T> {
    fn drop(&mut self) {
        self.__inner.unlock();
    }
}

impl<'a, T> Deref for FutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.ele
    }
}

impl<'a, T> DerefMut for FutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.ele
    }
}

pub struct Futex<T> {
    __inner: RawFutex,
    element: UnsafeCell<T>,
}

unsafe impl<T> Sync for Futex<T> {}

impl<T> Futex<T> {
    pub fn lock(&self) -> FutexGuard<T> {
        self.__inner.lock();
        unsafe {
            FutexGuard {
                __inner: &self.__inner,
                ele: &mut *self.element.get(),
            }
        }
    }

    pub fn new(x: T) -> Self {
        Futex {
            __inner: RawFutex {
                locked: AtomicBool::new(false),
                waiter: AtomicUsize::new(0),
            },
            element: UnsafeCell::new(x),
        }
    }
}

#[cfg(test)]
mod test {
    use alloc::sync::Arc;
    use alloc::vec::Vec;
    use std::time::Duration;

    #[test]
    fn futex() {
        let mutex = Arc::new(super::Futex::new(0));
        let mut vec = Vec::new();
        for i in 0..1000 {
            let mutex = mutex.clone();
            vec.push(std::thread::spawn(move || {
                std::thread::sleep(Duration::from_secs(1));
                let mut m = mutex.lock();
                *m += 1;
            }))
        }
        for i in vec {
            i.join().unwrap();
        }
        let v = mutex.lock();
        assert_eq!(*v, 1000);
    }
}