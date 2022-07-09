#![no_std]
#![no_main]

use core::{panic::PanicInfo, arch::global_asm};

#[cfg(feature = "board_raspi4")]
#[path = "boards/raspi4.rs"]
mod board;
#[cfg(not(any(feature = "board_raspi4")))]
#[path = "boards/qemu.rs"]
mod board;
mod driver;

use driver::uart;

global_asm!(include_str!("boot/boot.S"));

pub const LOGO: &str = include_str!("logo.txt");

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
extern "C" fn rust_main() {
    uart::uart_init();
    uart::uart_write_text(LOGO);
    uart::uart_write_text("\n");
    uart::uart_write_text("uart init......\n");
    loop{}
}