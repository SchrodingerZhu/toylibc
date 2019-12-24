use core::cell::UnsafeCell;
use core::intrinsics::*;
use core::ops::*;

use syscalls::*;

use crate::constants::*;

#[repr(C)]
pub struct RawFutex(pub u8, pub usize);

impl RawFutex {
    #[inline(always)]
    pub fn wait(&self) {
        unsafe {
            for _ in 0..100 {
                if atomic_load(&self.1 as *const _) == 0usize && atomic_load(&self.0 as *const _) == 0u8 {
                    return;
                }
                crate::thread_yield();
                core::sync::atomic::spin_loop_hint();
            }
            atomic_xadd(&self.1 as *const _ as *mut _, 1);
            match syscall4(SYS_futex, &self.0 as *const u8 as _, (FUTEX_WAIT | FUTEX_PRIVATE_FLAG) as _, 1, 0) {
                _ => ()
            }
            atomic_xsub(&self.1 as *const _ as *mut _, 1);
        }
    }

    #[inline(always)]
    pub fn lock(&self) {
        self.wait();
        unsafe {
            while let (_, false) = atomic_cxchg(&self.0 as *const _ as *mut _, 0, 1) {
                crate::thread_yield();
                core::sync::atomic::spin_loop_hint();
            }
        }
    }

    #[inline(always)]
    pub fn unlock(&self) {
        unsafe {
            atomic_store(&self.0 as *const _ as *mut _, 0);
            match syscall3(SYS_futex, &self.0 as *const _ as _, (FUTEX_WAKE | FUTEX_PRIVATE_FLAG) as _, 1) {
                _ => ()
            }
        }
    }
}

#[repr(C)]
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

#[repr(C)]
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
            __inner: RawFutex(0, 0),
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
    fn futex_1() {
        let mutex = Arc::new(super::Futex::new(0));
        let mut vec = Vec::new();
        for _ in 0..1000 {
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

    #[test]
    fn futex_2() {
        let mutex = Arc::new(super::Futex::new(0));
        let mut vec = Vec::new();
        for _ in 0..10 {
            let mutex = mutex.clone();
            vec.push(std::thread::spawn(move || {
                let mut m = mutex.lock();
                std::thread::sleep(Duration::from_millis(300));
                *m += 1;
            }))
        }
        for i in vec {
            i.join().unwrap();
        }
        let v = mutex.lock();
        assert_eq!(*v, 10);
    }
}