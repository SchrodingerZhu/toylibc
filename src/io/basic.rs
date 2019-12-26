#![allow(non_camel_case_types)]

use syscalls::*;

use crate::c_style::*;
use crate::types::*;

#[repr(C)]
pub struct _IO_FILE {
    _IO_read_ptr: *const u8,
    /* Current read pointer */
    _IO_read_end: *const u8,
    /* End of get area. */
    _IO_read_base: *const u8,
    /* Start of putback and get area. */
    _IO_write_base: *mut u8,
    /* Start of put area. */
    _IO_write_ptr: *mut u8,
    /* Current put pointer. */
    _IO_write_end: *mut u8,
    /* End of put area. */
    _IO_buf_base: *mut u8,
    /* Start of reserve area. */
    _IO_buf_end: *mut u8,
    /* End of reserve area. */
    _fileno: int_t,
    _blksize: int_t,
}


#[repr(C)]
pub struct iovec {
    io_base: *mut u8,
    size: size_t,
}

#[no_mangle]
pub unsafe extern "C" fn write(fd: int_t, buf: *const u8, cnt: ssize_t) -> ssize_t {
    match syscall3(SYS_write, fd as _, buf as _, cnt as _) {
        Ok(t) => t,
        Err(t) => {
            errno = t as _;
            -1
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn writev(fd: int_t, vs: *const iovec, vcnt: int_t) -> ssize_t {
    match syscall3(SYS_writev, fd as _, vs as _, vcnt as _) {
        Ok(t) => t,
        Err(t) => {
            errno = t as _;
            -1
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn writev() {
        let a = "HELLO, WORLD\n";
        unsafe {
            let iov = [super::iovec { io_base: a.as_ptr() as *mut u8, size: a.len() as _ },
                super::iovec { io_base: a.as_ptr() as *mut u8, size: a.len() as _ }];
            let v = super::writev(1, &iov as *const _, 2);
            println!("written: {} bytes", v);
            assert_eq!(v, 26);
        }
    }

    #[test]
    fn write() {
        let a = "HELLO, WORLD\n";
        unsafe {
            let v = super::write(1, a.as_ptr(), a.len() as _);
            println!("written: {} bytes", v);
            assert_eq!(v, 13);
        }
    }
}