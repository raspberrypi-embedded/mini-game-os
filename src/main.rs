#![no_std]
#![no_main]

use core::{panic::PanicInfo, arch::global_asm};
use bcm2711::mini_uart::uart_write_text;

#[cfg(feature = "board_qemu")]
#[path = "boards/qemu"]
mod board;
#[cfg(not(any(feature = "board_qemu")))]
#[path = "boards/raspi4/mod.rs"]
mod board;


global_asm!(include_str!("boot/boot.S"));

pub const LOGO: &'static str = include_str!("logo.txt");

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
extern "C" fn rust_main() {
    board::driver::uart_init();
    uart_write_text(LOGO);
    uart_write_text("uart init......\n");
    loop{}
}