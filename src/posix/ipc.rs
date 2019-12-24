use syscalls::*;

use crate::c_style::*;
use crate::constants::*;
use crate::shmid_ds;
use crate::types::*;

macro_rules! IPC_CMD {
    ($cmd:expr) => ((($cmd) & !IPC_TIME64) | IPC_64)
}

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
    match syscall1(SYS_shmdt, addr as _) {
        Ok(id) => id as _,
        Err(code) => {
            errno = code as _;
            -1
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn shmctl(id: int_t, cmd: int_t, buf: *mut shmid_ds) -> int_t {
    match syscall3(SYS_shmctl, id as _, IPC_CMD!(cmd) as _, buf as _) {
        Ok(id) => id as _,
        Err(code) => {
            errno = code as _;
            -1
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn shmat(id: int_t, addr: *const void_t, flag: int_t) -> *const void_t {
    match syscall3(SYS_shmat, id as _, addr as _, flag as _) {
        Ok(id) => id as _,
        Err(code) => {
            errno = code as _;
            (-1) as _
        }
    }
}
