use syscalls::*;

use crate::int_t;

#[inline(always)]
pub unsafe extern "C" fn cpu_relax() {
    asm!("pause\n" : : : "memory" : "volatile")
}

#[inline(always)]
pub unsafe extern "C" fn thread_yield() -> int_t {
    match syscall0(SYS_sched_yield) {
        Ok(_) => 0,
        Err(k) => k as i32
    }
}