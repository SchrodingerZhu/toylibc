use syscalls::*;

pub fn get() -> usize {
    unsafe {
        let ans = syscall0(SYS_gettid);
        ans.unwrap() as usize
    }
}