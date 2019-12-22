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

#[cfg(test)]
mod test {}