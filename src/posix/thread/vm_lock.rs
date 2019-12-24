use crate::utils::futex::RawFutex;

static mut VM_LOCK: RawFutex = RawFutex(0, 0);

#[no_mangle]
pub unsafe extern "C" fn __vm_wait() {
    VM_LOCK.wait();
}


#[no_mangle]
pub unsafe extern "C" fn __vm_lock() {
    VM_LOCK.lock();
}

#[no_mangle]
pub unsafe extern "C" fn __vm_unlock() {
    VM_LOCK.unlock();
}