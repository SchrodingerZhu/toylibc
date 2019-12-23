#[cfg(not(test))]
use core::panic::PanicInfo;

// This function is called on panic.

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


