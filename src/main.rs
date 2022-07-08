#![no_std]
#![no_main]

use core::{panic::PanicInfo, arch::global_asm};

global_asm!(include_str!("boot/boot.S"));

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
extern "C" fn rust_main() {
    loop{}
}