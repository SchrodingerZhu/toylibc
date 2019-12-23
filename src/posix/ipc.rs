use syscalls::*;

use crate::c_style::*;
use crate::constants::*;
use crate::types::*;

#[no_mangle]
pub unsafe extern "C" fn shmget(key: key_t, mut size: size_t, flag: int_t) -> int_t {
    if size > (PTRDIFF_MAX as _) {
        size = SIZE_MAX;
    }
    match syscall3(SYS_shmget, key as _, size, flag as _) {
        Ok(id) => id as _,
        Err(code) => {
            errno = code as _;
            -1
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn shmdt(addr: *mut u8) -> int_t {
    match syscall1(SYS_shmget, addr as _) {
        Ok(id) => id as _,
        Err(code) => {
            errno = code as _;
            -1
        }
    }
}