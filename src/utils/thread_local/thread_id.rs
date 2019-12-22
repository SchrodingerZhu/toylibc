use syscalls::*;

pub fn get() -> usize {
    unsafe {
        syscall0(SYS_getpid).unwrap() as usize
    }
}