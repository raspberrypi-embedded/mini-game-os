use core::ptr;

const VGA: [u32; 16] = [
    0x000000,
    0x0000AA,
    0x00AA00,
    0x00AAAA,
    0xAA0000,
    0xAA00AA,
    0xAA5500,
    0xAAAAAA,
    0x555555,
    0x5555FF,
    0x55FF55,
    0x55FFFF,
    0xFF5555,
    0xFF55FF,
    0xFFFF55,
    0xFFFFFF
];
pub struct Graphics {
    framebuffer: *mut u32,
    pitch: u32
}

impl Graphics {
    pub fn new(addr: *mut u32, pitch: u32) -> Self {
        Self {
            framebuffer: addr,
            pitch
        }
    }   

    pub fn draw_pixel(&self, x: u32, y: u32, attr: u8) {
        // let offset = y * self.pitch + x * 4;
        let offset = y * self.pitch / 4 + x;
        unsafe{
            ptr::write(
                self.framebuffer.add(offset as usize), 
                VGA[(attr & 0xf) as usize]
            );
        }
    }
}