#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

use core::{panic::PanicInfo, arch::global_asm};

use crate::{snake::Snake, graphics::Graphics};

extern crate alloc;

#[cfg(feature = "board_qemu")]
#[path = "boards/qemu/mod.rs"]
mod board;
#[cfg(not(any(feature = "board_qemu")))]
#[path = "boards/raspi4/mod.rs"]
mod board;

#[macro_use]
mod mm;
mod printf;
mod graphics;
mod snake;

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
    #[cfg(feature = "board_qemu")]
    {   
        use bcm2837::mailboxes::MailBox;
        use board::driver::FrameBuffer;
        println!("Frame Buffer init......");
        let mut mailbox = MailBox::new();
        let mut frame_buffer = FrameBuffer::new(1920, 1024, &mut mailbox);
        if let Ok((addr, pitch)) = frame_buffer.init() {
            graphics = graphics::Graphics::new(addr, pitch, 1024, 768);

            // graphics.draw_text("Hello World!", 100, 50);
        }
        
    }   

    #[cfg(feature = "board_raspi4")]
    {   
        use bcm2711::mailboxes::MailBox;
        println!("Frame Buffer init......");
        let mut mailbox = MailBox::new();
        let mut frame_buffer = FrameBuffer::new(1024, 768, &mut mailbox);
        if let Ok((addr, pitch)) = frame_buffer.init() {
            graphics = graphics::Graphics::new(addr, pitch);
            graphics.draw_pixel(100, 100, 1);
        }    
    }   

    snake = Snake::new(5, &mut graphics);
    snake.init();
    snake.display();
}