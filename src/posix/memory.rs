use core::intrinsics::offset;

use syscalls::*;

use crate::c_style::*;
use crate::constants::*;
use crate::types::*;

#[no_mangle]
pub unsafe extern fn brk(addr: *const void_t) -> int_t {
    let temp = syscall1(SYS_brk, 0).unwrap();
    match syscall1(SYS_brk, addr as ulong_t) {
        Ok(ret) if ret != temp => 0,
        _ => {
            errno = ENOMEM;
            -1
        }
    }
}

#[no_mangle]
pub unsafe extern fn sbrk(increment: intptr_t) -> *const void_t {
    let oldbrk = syscall1(SYS_brk, 0).unwrap() as *const u8;
    if increment != 0 {
        let newbrk = offset(oldbrk, increment as isize);
        if let Ok(ret) = syscall1(SYS_brk, newbrk as _) {
            if ret != newbrk as _ {
                errno = ENOMEM;
                (0b11111111) as *const void_t
            } else {
                oldbrk as *const void_t
            }
        } else {
            oldbrk as *const void_t
        }
    } else {
        oldbrk as *const void_t
    }
}

#[no_mangle]
pub unsafe extern fn getrlimit(resource: int_t, rlim: *mut rlimit) -> int_t {
    match syscall2(SYS_getrlimit, resource as _, rlim as _) {
        Ok(res) => res as _,
        Err(code) => {
            errno = code as _;
            -1
        }
    }
}

#[no_mangle]
pub unsafe extern fn mmap(addr: *mut u8, length: size_t, prot: int_t, flags: int_t, fd: int_t, offset: off_t) -> *const void_t {
    match syscall6(SYS_mmap, addr as _, length as _, prot as _, flags as _, fd as _, offset as _) {
        Ok(res) => res as _,
        Err(code) => {
            errno = code as _;
            (0b11111111) as *const void_t
        }
    }
}

#[no_mangle]
pub unsafe extern fn munmap(addr: *const void_t, length: size_t) -> int_t {
    match syscall2(SYS_munmap, addr as ulong_t, length) {
        Ok(res) => res as _,
        Err(code) => {
            errno = code as _;
            -1
        }
    }
}

#[cfg(test)]
mod test {}