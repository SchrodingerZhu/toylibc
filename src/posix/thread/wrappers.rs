#[no_mangle]
pub unsafe extern "C" fn thrd_yield() {
    crate::utils::thread_yield();
}

