// Copyright 2016 Amanieu d'Antras
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.


use lock_api;

use crate::utils::parking_lot::raw_rwlock::RawRwLock;

pub type RwLock<T> = lock_api::RwLock<RawRwLock, T>;

/// RAII structure used to release the shared read access of a lock when
/// dropped.
pub type RwLockReadGuard<'a, T> = lock_api::RwLockReadGuard<'a, RawRwLock, T>;

/// RAII structure used to release the exclusive write access of a lock when
/// dropped.
pub type RwLockWriteGuard<'a, T> = lock_api::RwLockWriteGuard<'a, RawRwLock, T>;

/// An RAII read lock guard returned by `RwLockReadGuard::map`, which can point to a
/// subfield of the protected data.
///
/// The main difference between `MappedRwLockReadGuard` and `RwLockReadGuard` is that the
/// former doesn't support temporarily unlocking and re-locking, since that
/// could introduce soundness issues if the locked object is modified by another
/// thread.
pub type MappedRwLockReadGuard<'a, T> = lock_api::MappedRwLockReadGuard<'a, RawRwLock, T>;

/// An RAII write lock guard returned by `RwLockWriteGuard::map`, which can point to a
/// subfield of the protected data.
///
/// The main difference between `MappedRwLockWriteGuard` and `RwLockWriteGuard` is that the
/// former doesn't support temporarily unlocking and re-locking, since that
/// could introduce soundness issues if the locked object is modified by another
/// thread.
pub type MappedRwLockWriteGuard<'a, T> = lock_api::MappedRwLockWriteGuard<'a, RawRwLock, T>;

/// RAII structure used to release the upgradable read access of a lock when
/// dropped.
pub type RwLockUpgradableReadGuard<'a, T> = lock_api::RwLockUpgradableReadGuard<'a, RawRwLock, T>;

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::mpsc::channel;
    use std::thread;
    use std::time::Duration;

    use rand::Rng;

    #[cfg(feature = "serde")]
    use bincode::{deserialize, serialize};

    use super::{RwLock, RwLockUpgradableReadGuard, RwLockWriteGuard};

    #[derive(Eq, PartialEq, Debug)]
    struct NonCopy(i32);

    #[test]
    fn smoke() {
        let l = RwLock::new(());
        drop(l.read());
        drop(l.write());
        drop(l.upgradable_read());
        drop((l.read(), l.read()));
        drop((l.read(), l.upgradable_read()));
        drop(l.write());
    }

    #[test]
    fn frob() {
        const N: u32 = 10;
        const M: u32 = 1000;

        let r = Arc::new(RwLock::new(()));

        let (tx, rx) = channel::<()>();
        for _ in 0..N {
            let tx = tx.clone();
            let r = r.clone();
            thread::spawn(move || {
                let mut rng = rand::XorShiftRng::new_unseeded();
                for _ in 0..M {
                    if rng.gen() {
                        drop(r.write());
                    } else {
                        drop(r.read());
                    }
                }
                drop(tx);
            });
        }
        drop(tx);
        let _ = rx.recv();
    }

    #[test]
    fn test_rw_arc_no_poison_wr() {
        let arc = Arc::new(RwLock::new(1));
        let arc2 = arc.clone();
        let _: Result<(), _> = thread::spawn(move || {
            let _lock = arc2.write();
            panic!();
        })
            .join();
        let lock = arc.read();
        assert_eq!(*lock, 1);
    }

    #[test]
    fn test_rw_arc_no_poison_ww() {
        let arc = Arc::new(RwLock::new(1));
        let arc2 = arc.clone();
        let _: Result<(), _> = thread::spawn(move || {
            let _lock = arc2.write();
            panic!();
        })
            .join();
        let lock = arc.write();
        assert_eq!(*lock, 1);
    }

    #[test]
    fn test_rw_arc_no_poison_rr() {
        let arc = Arc::new(RwLock::new(1));
        let arc2 = arc.clone();
        let _: Result<(), _> = thread::spawn(move || {
            let _lock = arc2.read();
            panic!();
        })
            .join();
        let lock = arc.read();
        assert_eq!(*lock, 1);
    }

    #[test]
    fn test_rw_arc_no_poison_rw() {
        let arc = Arc::new(RwLock::new(1));
        let arc2 = arc.clone();
        let _: Result<(), _> = thread::spawn(move || {
            let _lock = arc2.read();
            panic!()
        })
            .join();
        let lock = arc.write();
        assert_eq!(*lock, 1);
    }

    #[test]
    fn test_ruw_arc() {
        let arc = Arc::new(RwLock::new(0));
        let arc2 = arc.clone();
        let (tx, rx) = channel();

        thread::spawn(move || {
            for _ in 0..10 {
                let mut lock = arc2.write();
                let tmp = *lock;
                *lock = -1;
                thread::yield_now();
                *lock = tmp + 1;
            }
            tx.send(()).unwrap();
        });

        let mut children = Vec::new();

        // Upgradable readers try to catch the writer in the act and also
        // try to touch the value
        for _ in 0..5 {
            let arc3 = arc.clone();
            children.push(thread::spawn(move || {
                let lock = arc3.upgradable_read();
                let tmp = *lock;
                assert!(tmp >= 0);
                thread::yield_now();
                let mut lock = RwLockUpgradableReadGuard::upgrade(lock);
                assert_eq!(tmp, *lock);
                *lock = -1;
                thread::yield_now();
                *lock = tmp + 1;
            }));
        }

        // Readers try to catch the writers in the act
        for _ in 0..5 {
            let arc4 = arc.clone();
            children.push(thread::spawn(move || {
                let lock = arc4.read();
                assert!(*lock >= 0);
            }));
        }

        // Wait for children to pass their asserts
        for r in children {
            assert!(r.join().is_ok());
        }

        // Wait for writer to finish
        rx.recv().unwrap();
        let lock = arc.read();
        assert_eq!(*lock, 15);
    }

    #[test]
    fn test_rw_arc() {
        let arc = Arc::new(RwLock::new(0));
        let arc2 = arc.clone();
        let (tx, rx) = channel();

        thread::spawn(move || {
            let mut lock = arc2.write();
            for _ in 0..10 {
                let tmp = *lock;
                *lock = -1;
                thread::yield_now();
                *lock = tmp + 1;
            }
            tx.send(()).unwrap();
        });

        // Readers try to catch the writer in the act
        let mut children = Vec::new();
        for _ in 0..5 {
            let arc3 = arc.clone();
            children.push(thread::spawn(move || {
                let lock = arc3.read();
                assert!(*lock >= 0);
            }));
        }

        // Wait for children to pass their asserts
        for r in children {
            assert!(r.join().is_ok());
        }

        // Wait for writer to finish
        rx.recv().unwrap();
        let lock = arc.read();
        assert_eq!(*lock, 10);
    }

    #[test]
    fn test_rw_arc_access_in_unwind() {
        let arc = Arc::new(RwLock::new(1));
        let arc2 = arc.clone();
        let _ = thread::spawn(move || {
            struct Unwinder {
                i: Arc<RwLock<isize>>,
            }
            impl Drop for Unwinder {
                fn drop(&mut self) {
                    let mut lock = self.i.write();
                    *lock += 1;
                }
            }
            let _u = Unwinder { i: arc2 };
            panic!();
        })
            .join();
        let lock = arc.read();
        assert_eq!(*lock, 2);
    }

    #[test]
    fn test_rwlock_unsized() {
        let rw: &RwLock<[i32; 3]> = &RwLock::new([1, 2, 3]);
        {
            let b = &mut *rw.write();
            b[0] = 4;
            b[2] = 5;
        }
        let comp: &[i32] = &[4, 2, 5];
        assert_eq!(&*rw.read(), comp);
    }

    #[test]
    fn test_rwlock_try_read() {
        let lock = RwLock::new(0isize);
        {
            let read_guard = lock.read();

            let read_result = lock.try_read();
            assert!(
                read_result.is_some(),
                "try_read should succeed while read_guard is in scope"
            );

            drop(read_guard);
        }
        {
            let upgrade_guard = lock.upgradable_read();

            let read_result = lock.try_read();
            assert!(
                read_result.is_some(),
                "try_read should succeed while upgrade_guard is in scope"
            );

            drop(upgrade_guard);
        }
        {
            let write_guard = lock.write();

            let read_result = lock.try_read();
            assert!(
                read_result.is_none(),
                "try_read should fail while write_guard is in scope"
            );

            drop(write_guard);
        }
    }

    #[test]
    fn test_rwlock_try_write() {
        let lock = RwLock::new(0isize);
        {
            let read_guard = lock.read();

            let write_result = lock.try_write();
            assert!(
                write_result.is_none(),
                "try_write should fail while read_guard is in scope"
            );

            drop(read_guard);
        }
        {
            let upgrade_guard = lock.upgradable_read();

            let write_result = lock.try_write();
            assert!(
                write_result.is_none(),
                "try_write should fail while upgrade_guard is in scope"
            );

            drop(upgrade_guard);
        }
        {
            let write_guard = lock.write();

            let write_result = lock.try_write();
            assert!(
                write_result.is_none(),
                "try_write should fail while write_guard is in scope"
            );

            drop(write_guard);
        }
    }

    #[test]
    fn test_rwlock_try_upgrade() {
        let lock = RwLock::new(0isize);
        {
            let read_guard = lock.read();

            let upgrade_result = lock.try_upgradable_read();
            assert!(
                upgrade_result.is_some(),
                "try_upgradable_read should succeed while read_guard is in scope"
            );

            drop(read_guard);
        }
        {
            let upgrade_guard = lock.upgradable_read();

            let upgrade_result = lock.try_upgradable_read();
            assert!(
                upgrade_result.is_none(),
                "try_upgradable_read should fail while upgrade_guard is in scope"
            );

            drop(upgrade_guard);
        }
        {
            let write_guard = lock.write();

            let upgrade_result = lock.try_upgradable_read();
            assert!(
                upgrade_result.is_none(),
                "try_upgradable should fail while write_guard is in scope"
            );

            drop(write_guard);
        }
    }

    #[test]
    fn test_into_inner() {
        let m = RwLock::new(NonCopy(10));
        assert_eq!(m.into_inner(), NonCopy(10));
    }

    #[test]
    fn test_into_inner_drop() {
        struct Foo(Arc<AtomicUsize>);
        impl Drop for Foo {
            fn drop(&mut self) {
                self.0.fetch_add(1, Ordering::SeqCst);
            }
        }
        let num_drops = Arc::new(AtomicUsize::new(0));
        let m = RwLock::new(Foo(num_drops.clone()));
        assert_eq!(num_drops.load(Ordering::SeqCst), 0);
        {
            let _inner = m.into_inner();
            assert_eq!(num_drops.load(Ordering::SeqCst), 0);
        }
        assert_eq!(num_drops.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_get_mut() {
        let mut m = RwLock::new(NonCopy(10));
        *m.get_mut() = NonCopy(20);
        assert_eq!(m.into_inner(), NonCopy(20));
    }

    #[test]
    fn test_rwlockguard_sync() {
        fn sync<T: Sync>(_: T) {}

        let rwlock = RwLock::new(());
        sync(rwlock.read());
        sync(rwlock.write());
    }

    #[test]
    fn test_rwlock_downgrade() {
        let x = Arc::new(RwLock::new(0));
        let mut handles = Vec::new();
        for _ in 0..8 {
            let x = x.clone();
            handles.push(thread::spawn(move || {
                for _ in 0..100 {
                    let mut writer = x.write();
                    *writer += 1;
                    let cur_val = *writer;
                    let reader = RwLockWriteGuard::downgrade(writer);
                    assert_eq!(cur_val, *reader);
                }
            }));
        }
        for handle in handles {
            handle.join().unwrap()
        }
        assert_eq!(*x.read(), 800);
    }

    #[test]
    fn test_rwlock_recursive() {
        let arc = Arc::new(RwLock::new(1));
        let arc2 = arc.clone();
        let _lock1 = arc.read();
        thread::spawn(move || {
            let _lock = arc2.write();
        });

        if cfg!(not(all(target_env = "sgx", target_vendor = "fortanix"))) {
            thread::sleep(Duration::from_millis(100));
        } else {
            // FIXME: https://github.com/fortanix/rust-sgx/issues/31
            for _ in 0..100 {
                thread::yield_now();
            }
        }

        // A normal read would block here since there is a pending writer
        let _lock2 = arc.read_recursive();
    }

    #[test]
    fn test_rwlock_debug() {
        let x = RwLock::new(vec![0u8, 10]);

        assert_eq!(format!("{:?}", x), "RwLock { data: [0, 10] }");
        let _lock = x.write();
        assert_eq!(format!("{:?}", x), "RwLock { data: <locked> }");
    }

    #[test]
    fn test_clone() {
        let rwlock = RwLock::new(Arc::new(1));
        let a = rwlock.read_recursive();
        let b = a.clone();
        assert_eq!(Arc::strong_count(&b), 2);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde() {
        let contents: Vec<u8> = vec![0, 1, 2];
        let mutex = RwLock::new(contents.clone());

        let serialized = serialize(&mutex).unwrap();
        let deserialized: RwLock<Vec<u8>> = deserialize(&serialized).unwrap();

        assert_eq!(*(mutex.read()), *(deserialized.read()));
        assert_eq!(contents, *(deserialized.read()));
    }
}
