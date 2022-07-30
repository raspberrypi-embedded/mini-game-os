#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

use core::{panic::PanicInfo, arch::global_asm};

use crate::{snake::Snake, graphics::Graphics};
use raspiberry_peripherals::mailboxes::MailBox;
use board::driver::FrameBuffer;

extern crate alloc;

#[cfg(feature = "board_qemu")]
#[path = "boards/raspi/mod.rs"]
mod board;
#[cfg(not(any(feature = "board_qemu")))]
#[path = "boards/raspi/mod.rs"]
mod board;



#[macro_use]
mod mm;
mod printf;
mod graphics;
mod snake;
mod timer;

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
    println!("Uart init......");
    mm::KERNEL_HEAP.mm_init();

    let mut graphics: Graphics = Graphics::uninit();
    let mut snake: snake::Snake;
    println!("Frame Buffer init......");
    let mut mailbox = MailBox::new();
    let mut frame_buffer = FrameBuffer::new(1080, 1920, &mut mailbox);
    if let Ok((addr, pitch)) = frame_buffer.init() {
        graphics = graphics::Graphics::new(addr, pitch,1080, 1920);
    }

    snake = Snake::new(10, &mut graphics);
    snake.init();
    snake.display();
    snake.play();
}