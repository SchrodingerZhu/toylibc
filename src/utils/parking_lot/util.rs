use core::time::Duration;

use crate::time::Instant;

// Option::unchecked_unwrap
pub trait UncheckedOptionExt<T> {
    unsafe fn unchecked_unwrap(self) -> T;
}

impl<T> UncheckedOptionExt<T> for Option<T> {
    #[inline]
    unsafe fn unchecked_unwrap(self) -> T {
        match self {
            Some(x) => x,
            None => unreachable(),
        }
    }
}

// hint::unreachable_unchecked() in release mode
#[inline]
unsafe fn unreachable() -> ! {
    if cfg!(debug_assertions) {
        unreachable!();
    } else {
        core::hint::unreachable_unchecked()
    }
}

#[inline]
pub fn to_deadline(timeout: Duration) -> Option<Instant> {
    Instant::now().checked_add(timeout)
}