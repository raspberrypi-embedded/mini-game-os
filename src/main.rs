#![no_std]
#![no_main]

use core::{panic::PanicInfo, arch::global_asm};
use bcm2711::mailboxes::MailBox;

#[cfg(feature = "board_qemu")]
#[path = "boards/qemu"]
mod board;
#[cfg(not(any(feature = "board_qemu")))]
#[path = "boards/raspi4/mod.rs"]
mod board;

#[macro_use]
mod mm;
mod printf;

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

    println!("{}", LOGO);
    println!("Uart init......\n");
    // loop{
    //     board::driver::uart_wait_read();
    // }
    let mut mail_box = MailBox::new();
    let mut frame_buffer = board::driver::FrameBuffer::new(32, 32, &mut mail_box);
    frame_buffer.init();
}