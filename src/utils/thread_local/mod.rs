use alloc::boxed::Box;
use alloc::vec;
use core::cell::UnsafeCell;
use core::fmt;
use core::marker::PhantomData;
use core::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};

use spin::Mutex;

pub use cached::{CachedIntoIter, CachedIterMut, CachedThreadLocal};
use unreachable::{UncheckedOptionExt, UncheckedResultExt};

mod thread_id;
pub(crate) mod unreachable;
mod cached;

/// Thread-local variable wrapper
///
/// See the [module-level documentation](index.html) for more.
pub struct ThreadLocal<T: Send> {
    // Pointer to the current top-level hash table
    table: AtomicPtr<Table<T>>,

    // Lock used to guard against concurrent modifications. This is only taken
    // while writing to the table, not when reading from it. This also guards
    // the counter for the total number of values in the hash table.
    lock: Mutex<usize>,
}

struct Table<T: Send> {
    // Hash entries for the table
    entries: Box<[TableEntry<T>]>,

    // Number of bits used for the hash function
    hash_bits: usize,

    // Previous table, half the size of the current one
    prev: Option<Box<Table<T>>>,
}

struct TableEntry<T: Send> {
    // Current owner of this entry, or 0 if this is an empty entry
    owner: AtomicUsize,

    // The object associated with this entry. This is only ever accessed by the
    // owner of the entry.
    data: UnsafeCell<Option<Box<T>>>,
}

// ThreadLocal is always Sync, even if T isn't
unsafe impl<T: Send> Sync for ThreadLocal<T> {}

impl<T: Send> Default for ThreadLocal<T> {
    fn default() -> ThreadLocal<T> {
        ThreadLocal::new()
    }
}

impl<T: Send> Drop for ThreadLocal<T> {
    fn drop(&mut self) {
        unsafe {
            Box::from_raw(self.table.load(Ordering::Relaxed));
        }
    }
}

// Implementation of Clone for TableEntry, needed to make vec![] work
impl<T: Send> Clone for TableEntry<T> {
    fn clone(&self) -> TableEntry<T> {
        TableEntry {
            owner: AtomicUsize::new(0),
            data: UnsafeCell::new(None),
        }
    }
}

// Hash function for the thread id
#[cfg(target_pointer_width = "32")]
#[inline]
fn hash(id: usize, bits: usize) -> usize {
    id.wrapping_mul(0x9E3779B9) >> (32 - bits)
}

#[cfg(target_pointer_width = "64")]
#[inline]
fn hash(id: usize, bits: usize) -> usize {
    id.wrapping_mul(0x9E37_79B9_7F4A_7C15) >> (64 - bits)
}

impl<T: Send> ThreadLocal<T> {
    /// Creates a new empty `ThreadLocal`.
    pub fn new() -> ThreadLocal<T> {
        let entry = TableEntry {
            owner: AtomicUsize::new(0),
            data: UnsafeCell::new(None),
        };
        let table = Table {
            entries: vec![entry; 2].into_boxed_slice(),
            hash_bits: 1,
            prev: None,
        };
        ThreadLocal {
            table: AtomicPtr::new(Box::into_raw(Box::new(table))),
            lock: Mutex::new(0),
        }
    }

    /// Returns the element for the current thread, if it exists.
    pub fn get(&self) -> Option<&T> {
        let id = thread_id::get();
        self.get_fast(id)
    }

    /// Returns the element for the current thread, or creates it if it doesn't
    /// exist.
    pub fn get_or<F>(&self, create: F) -> &T
        where
            F: FnOnce() -> T,
    {
        unsafe {
            self.get_or_try(|| Ok::<T, ()>(create()))
                .unchecked_unwrap_ok()
        }
    }

    /// Returns the element for the current thread, or creates it if it doesn't
    /// exist. If `create` fails, that error is returned and no element is
    /// added.
    pub fn get_or_try<F, E>(&self, create: F) -> Result<&T, E>
        where
            F: FnOnce() -> Result<T, E>,
    {
        let id = thread_id::get();
        match self.get_fast(id) {
            Some(x) => Ok(x),
            None => Ok(self.insert(id, Box::new(create()?), true)),
        }
    }

    // Simple hash table lookup function
    fn lookup(id: usize, table: &Table<T>) -> Option<&UnsafeCell<Option<Box<T>>>> {
        // Because we use a Mutex to prevent concurrent modifications (but not
        // reads) of the hash table, we can avoid any memory barriers here. No
        // elements between our hash bucket and our value can have been modified
        // since we inserted our thread-local value into the table.
        for entry in table.entries.iter().cycle().skip(hash(id, table.hash_bits)) {
            let owner = entry.owner.load(Ordering::Relaxed);
            if owner == id {
                return Some(&entry.data);
            }
            if owner == 0 {
                return None;
            }
        }
        unreachable!();
    }

    // Fast path: try to find our thread in the top-level hash table
    fn get_fast(&self, id: usize) -> Option<&T> {
        let table = unsafe { &*self.table.load(Ordering::Relaxed) };
        match Self::lookup(id, table) {
            Some(x) => unsafe { Some((*x.get()).as_ref().unchecked_unwrap()) },
            None => self.get_slow(id, table),
        }
    }

    // Slow path: try to find our thread in the other hash tables, and then
    // move it to the top-level hash table.
    #[cold]
    fn get_slow(&self, id: usize, table_top: &Table<T>) -> Option<&T> {
        let mut current = &table_top.prev;
        while let Some(ref table) = *current {
            if let Some(x) = Self::lookup(id, table) {
                let data = unsafe { (*x.get()).take().unchecked_unwrap() };
                return Some(self.insert(id, data, false));
            }
            current = &table.prev;
        }
        None
    }

    #[cold]
    fn insert(&self, id: usize, data: Box<T>, new: bool) -> &T {
        // Lock the Mutex to ensure only a single thread is modify the hash
        // table at once.
        let mut count = self.lock.lock();
        if new {
            *count += 1;
        }
        let table_raw = self.table.load(Ordering::Relaxed);
        let table = unsafe { &*table_raw };

        // If the current top-level hash table is more than 75% full, add a new
        // level with 2x the capacity. Elements will be moved up to the new top
        // level table as they are accessed.
        let table = if *count > table.entries.len() * 3 / 4 {
            let entry = TableEntry {
                owner: AtomicUsize::new(0),
                data: UnsafeCell::new(None),
            };
            let new_table = Box::into_raw(Box::new(Table {
                entries: vec![entry; table.entries.len() * 2].into_boxed_slice(),
                hash_bits: table.hash_bits + 1,
                prev: unsafe { Some(Box::from_raw(table_raw)) },
            }));
            self.table.store(new_table, Ordering::Release);
            unsafe { &*new_table }
        } else {
            table
        };

        // Insert the new element into the top-level hash table
        for entry in table.entries.iter().cycle().skip(hash(id, table.hash_bits)) {
            let owner = entry.owner.load(Ordering::Relaxed);
            if owner == 0 {
                unsafe {
                    entry.owner.store(id, Ordering::Relaxed);
                    *entry.data.get() = Some(data);
                    return (*entry.data.get()).as_ref().unchecked_unwrap();
                }
            }
            if owner == id {
                // This can happen if create() inserted a value into this
                // ThreadLocal between our calls to get_fast() and insert(). We
                // just return the existing value and drop the newly-allocated
                // Box.
                unsafe {
                    return (*entry.data.get()).as_ref().unchecked_unwrap();
                }
            }
        }
        unreachable!();
    }

    fn raw_iter(&mut self) -> RawIter<T> {
        RawIter {
            remaining: *self.lock.lock(),
            index: 0,
            table: self.table.load(Ordering::Relaxed),
        }
    }

    /// Returns a mutable iterator over the local values of all threads.
    ///
    /// Since this call borrows the `ThreadLocal` mutably, this operation can
    /// be done safely---the mutable borrow statically guarantees no other
    /// threads are currently accessing their associated values.
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            raw: self.raw_iter(),
            marker: PhantomData,
        }
    }

    /// Removes all thread-specific values from the `ThreadLocal`, effectively
    /// reseting it to its original state.
    ///
    /// Since this call borrows the `ThreadLocal` mutably, this operation can
    /// be done safely---the mutable borrow statically guarantees no other
    /// threads are currently accessing their associated values.
    pub fn clear(&mut self) {
        *self = ThreadLocal::new();
    }
}

impl<T: Send> IntoIterator for ThreadLocal<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(mut self) -> IntoIter<T> {
        IntoIter {
            raw: self.raw_iter(),
            _thread_local: self,
        }
    }
}

impl<'a, T: Send + 'a> IntoIterator for &'a mut ThreadLocal<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> IterMut<'a, T> {
        self.iter_mut()
    }
}

impl<T: Send + Default> ThreadLocal<T> {
    /// Returns the element for the current thread, or creates a default one if
    /// it doesn't exist.
    pub fn get_or_default(&self) -> &T {
        self.get_or(Default::default)
    }
}

impl<T: Send + fmt::Debug> fmt::Debug for ThreadLocal<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ThreadLocal {{ local_data: {:?} }}", self.get())
    }
}


struct RawIter<T: Send> {
    remaining: usize,
    index: usize,
    table: *const Table<T>,
}

impl<T: Send> Iterator for RawIter<T> {
    type Item = *mut Option<Box<T>>;

    fn next(&mut self) -> Option<*mut Option<Box<T>>> {
        if self.remaining == 0 {
            return None;
        }

        loop {
            let entries = unsafe { &(*self.table).entries[..] };
            while self.index < entries.len() {
                let val = entries[self.index].data.get();
                self.index += 1;
                if unsafe { (*val).is_some() } {
                    self.remaining -= 1;
                    return Some(val);
                }
            }
            self.index = 0;
            self.table = unsafe { &**(*self.table).prev.as_ref().unchecked_unwrap() };
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

/// Mutable iterator over the contents of a `ThreadLocal`.
pub struct IterMut<'a, T: Send + 'a> {
    raw: RawIter<T>,
    marker: PhantomData<&'a mut ThreadLocal<T>>,
}

impl<'a, T: Send + 'a> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<&'a mut T> {
        self.raw
            .next()
            .map(|x| unsafe { &mut **(*x).as_mut().unchecked_unwrap() })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.raw.size_hint()
    }
}

impl<'a, T: Send + 'a> ExactSizeIterator for IterMut<'a, T> {}

/// An iterator that moves out of a `ThreadLocal`.
pub struct IntoIter<T: Send> {
    raw: RawIter<T>,
    _thread_local: ThreadLocal<T>,
}

impl<T: Send> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.raw
            .next()
            .map(|x| unsafe { *(*x).take().unchecked_unwrap() })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.raw.size_hint()
    }
}

impl<T: Send> ExactSizeIterator for IntoIter<T> {}