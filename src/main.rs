#![no_std]
#![no_main]

use core::{panic::PanicInfo, arch::global_asm};

use crate::{board::driver::FrameBuffer, graphics::Graphics};

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
    #[cfg(feature = "board_qemu")]
    {   
        use bcm2837::mailboxes::MailBox;
        println!("Frame Buffer init......");
        let mut mailbox = MailBox::new();
        let mut frame_buffer = FrameBuffer::new(1024, 768, &mut mailbox);
        if let Ok((addr, pitch)) = frame_buffer.init() {
            let graphics = graphics::Graphics::new(addr, pitch);
            // graphics.draw_pixel(100, 100, 1);
            for i in 100..200 {
                println!("[Debug] draw pixel ({}, {})", i, 100);
                graphics.draw_pixel(i, 100, 0x0C);
            }
        }
        
    }   

    #[cfg(feature = "board_raspi4")]
    {   
        use bcm2711::mailboxes::MailBox;
        println!("Frame Buffer init......");
        let mut mailbox = MailBox::new();
        let mut frame_buffer = FrameBuffer::new(1024, 768, &mut mailbox);
        if let Ok((addr, pitch)) = frame_buffer.init() {
            let graphics = graphics::Graphics::new(addr, pitch);
            graphics.draw_pixel(100, 100, 1);
        }    
    }   
}