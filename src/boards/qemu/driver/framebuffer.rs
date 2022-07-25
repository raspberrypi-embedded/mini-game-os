use bcm2837::mailboxes::*;
use crate::println;

// pub static mut FRAME_BUFFER: FrameBuffer = FrameBuffer::new(0, 0);


pub struct FrameBuffer<'a> {
    height: u32,
    width: u32,
    mail_box: &'a mut MailBox
}

impl<'a> FrameBuffer<'a> {
    pub fn new(height: u32, width: u32, mail_box: &'a mut MailBox) -> Self {
        Self{
            height, 
            width,
            mail_box
        }
    }

    /// (ref: https://github.com/isometimes/rpi4-osdev/blob/master/part5-framebuffer/fb.c)
    pub fn init(&mut self) {
        println!("[Debug] Raspberry Pi Mailbox init......\n");
        // Length of message in bytes
        self.mail_box.write_buf(0, 35 * 4);
        self.mail_box.write_buf(1, MAILBOX_REQUEST);

        println!("[Debug] Set physical width");
        // Tag identifier
        self.mail_box.write_buf(2, MailboxTag::SetPhyWH as u32);
        // Value size in bytes
        self.mail_box.write_buf(3, 8);
        self.mail_box.write_buf(4, 0);
        // Value(width)
        self.mail_box.write_buf(5, self.width);
        // Value(height)
        self.mail_box.write_buf(6, self.height);

        // Set Virtual height/width
        println!("[Debug] Set virtual width");
        self.mail_box.write_buf(7, MailboxTag::SetVirtWH as u32);
        self.mail_box.write_buf(8, 8);
        self.mail_box.write_buf(9, 8);
        self.mail_box.write_buf(10, self.width);
        self.mail_box.write_buf(11, self.height);

        // Set Virt Offset
        println!("[Debug] Set virtual offset");
        self.mail_box.write_buf(12,MailboxTag::SetVirtOff as u32);
        self.mail_box.write_buf(13, 8);
        self.mail_box.write_buf(14, 8);
        self.mail_box.write_buf(15, 0);
        self.mail_box.write_buf(16, 0);

        println!("[Debug] Set rgb depth");
        self.mail_box.write_buf(17, MailboxTag::SetDepth as u32);
        self.mail_box.write_buf(18, 4);
        self.mail_box.write_buf(19, 4);
        self.mail_box.write_buf(20, 32);

        println!("[Debug] Set pixel order");
        self.mail_box.write_buf(21, MailboxTag::SetPixelOrder as u32);
        self.mail_box.write_buf(22, 4);
        self.mail_box.write_buf(23, 4);
        self.mail_box.write_buf(24, 1);

        println!("[Debug] Set framebuffer");
        self.mail_box.write_buf(25, MailboxTag::GetFB as u32);
        self.mail_box.write_buf(26, 8);
        self.mail_box.write_buf(27, 8);
        self.mail_box.write_buf(28, 4096);
        self.mail_box.write_buf(29, 0);

        println!("[Debug] Set pitch");
        self.mail_box.write_buf(30, MailboxTag::GetPitch as u32);
        self.mail_box.write_buf(31, 4);
        self.mail_box.write_buf(32, 4);
        self.mail_box.write_buf(33, 0);

        self.mail_box.write_buf(34, MailboxTag::TagLast as u32);

        println!("[Debug] mail box call");
        if self.mail_box.call(MailboxChannel::Property).is_ok() && self.mail_box.read_buf(20) == 32 && self.mail_box.read_buf(28) != 0 {
            self.mail_box.write_buf(28, 0x3FFFFFFF);
            let width = self.mail_box.read_buf(5);
            let height = self.mail_box.read_buf(6);
            let pitch = self.mail_box.read_buf(33);
            let isrgb = self.mail_box.read_buf(24);
            println!("[Debug] width: {}, height: {}, pitch: {}, isgrb: {}", width, height, pitch, isrgb);
        }else{
            println!("[Debug] mailbox[20]: {}, mailbox[32]: {}", self.mail_box.read_buf(20), self.mail_box.read_buf(32));
        }
    }
}