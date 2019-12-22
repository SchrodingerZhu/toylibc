use syscalls::*;

pub fn get() -> usize {
    unsafe {
        syscall0(SYS_gettid).unwrap() as usize
    }
}