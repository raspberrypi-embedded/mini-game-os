use core::ptr;
use core::convert::TryInto;
use embedded_graphics::geometry::{ OriginDimensions, Size };
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::{ Gray8, GrayColor, Rgb565, raw::RawU16 };
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::*;


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
    width: u32,
    height: u32,
    framebuffer: *mut u32,
    pitch: u32
}

impl Graphics {
    pub fn new(addr: *mut u32, pitch: u32, width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            framebuffer: addr,
            pitch
        }
    }   

    pub fn draw_pixel(&self, x: u32, y: u32, color: u16) {
        // let offset = y * self.pitch + x * 4;
        let offset = y * self.pitch / 4 + x;
        unsafe{
            ptr::write(
                self.framebuffer.add(offset as usize), 
                // VGA[(attr & 0xf) as usize]
                color as u32
            );
        }
    }

    pub fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        // Red 1 pixel wide line from (50, 20) to (60, 35)
        Line::new(Point::new(x1, y1), Point::new(x2, y2))
        .into_styled(PrimitiveStyle::with_stroke(Rgb565::RED, 1))
        .draw(self).unwrap();
    }
}

impl OriginDimensions for Graphics {
    fn size(&self) -> Size {
        Size::new(self.width, self.height)
    }
}

impl DrawTarget for Graphics {
    type Color = Rgb565;

    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            // Check if the pixel coordinates are out of bounds (negative or greater than
            // (63,63)). `DrawTarget` implementation are required to discard any out of bounds
            // pixels without returning an error or causing a panic.

            if let Ok((x, y)) = coord.try_into() {
                assert!(x <= self.width);
                assert!(y <= self.height);
                self.draw_pixel(x, y, RawU16::from(color).into_inner())
            }
        }

        Ok(())
    }
}