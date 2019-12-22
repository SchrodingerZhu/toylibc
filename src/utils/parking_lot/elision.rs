use core::sync::atomic::AtomicUsize;

pub trait AtomicElisionExt {
    type IntType;

    // Perform a compare_exchange and start a transaction
    fn elision_compare_exchange_acquire(
        &self,
        current: Self::IntType,
        new: Self::IntType,
    ) -> Result<Self::IntType, Self::IntType>;

    // Perform a fetch_sub and end a transaction
    fn elision_fetch_sub_release(&self, val: Self::IntType) -> Self::IntType;
}

impl AtomicElisionExt for AtomicUsize {
    type IntType = usize;

    #[inline]
    fn elision_compare_exchange_acquire(&self, current: usize, new: usize) -> Result<usize, usize> {
        unsafe {
            let prev: usize;
            asm!("xacquire; lock; cmpxchgq $2, $1"
                 : "={rax}" (prev), "+*m" (self)
                 : "r" (new), "{rax}" (current)
                 : "memory"
                 : "volatile");
            if prev == current {
                Ok(prev)
            } else {
                Err(prev)
            }
        }
    }

    #[inline]
    fn elision_fetch_sub_release(&self, val: usize) -> usize {
        unsafe {
            let prev: usize;
            asm!("xrelease; lock; xaddq $2, $1"
                 : "=r" (prev), "+*m" (self)
                 : "0" (val.wrapping_neg())
                 : "memory"
                 : "volatile");
            prev
        }
    }
}
