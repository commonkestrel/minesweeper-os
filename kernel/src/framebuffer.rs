use bootloader_api::info::{FrameBuffer, PixelFormat};
use embedded_graphics::{prelude::*, pixelcolor::Rgb888};

pub struct Display {
    framebuffer: &'static mut FrameBuffer,
}

impl Display {
    pub fn new(framebuffer: &'static mut FrameBuffer) -> Display {
        Self { framebuffer }
    }

    pub fn clear(&mut self) {
        self.framebuffer.buffer_mut().fill(0);
    }

    /// Caller must ensure that the x and y coordinates are both >= 0
    /// and less than the framebuffer width and height respectively.
    unsafe fn set_pixel(framebuffer: &mut FrameBuffer, x: i32, y: i32, r: u8, g: u8, b: u8) {
        let info = framebuffer.info();

        match info.pixel_format {
            PixelFormat::Rgb => {
                let index = info.stride * info.bytes_per_pixel * (y as usize) + info.bytes_per_pixel * (x as usize) + info.bytes_per_pixel - 3;
                framebuffer.buffer_mut()[index] = r;
                framebuffer.buffer_mut()[index + 1] = g;
                framebuffer.buffer_mut()[index + 2] = b;
            },
            PixelFormat::Bgr => {
                let index = info.stride * info.bytes_per_pixel * (y as usize) + info.bytes_per_pixel * (x as usize) + info.bytes_per_pixel - 3;
                framebuffer.buffer_mut()[index] = b;
                framebuffer.buffer_mut()[index + 1] = g;
                framebuffer.buffer_mut()[index + 2] = r;
            },
            PixelFormat::U8 => {
                let r16 = r as u16;
                let g16 = g as u16;
                let b16 = b as u16;
                let y = ((3*r16 + b16 + 4*g16)/8) as u8;

                let index = info.stride * info.bytes_per_pixel * (y as usize) + info.bytes_per_pixel * (x as usize) + info.bytes_per_pixel - 1;
                framebuffer.buffer_mut()[index] = y;
            },
            PixelFormat::Unknown { red_position, green_position, blue_position } => {
                let index = info.stride * info.bytes_per_pixel * (y as usize) + info.bytes_per_pixel * (x as usize);
                framebuffer.buffer_mut()[index + red_position as usize] = r;
                framebuffer.buffer_mut()[index + green_position as usize] = g;
                framebuffer.buffer_mut()[index + blue_position as usize] = b;
            },
            _ => {},
        }
        
    }

    fn draw_pixel(&mut self, Pixel(point, color): Pixel<Rgb888>) {
        let size = self.size();
        
        if (0..(size.width as i32)).contains(&point.x) && (0..(size.height as i32)).contains(&point.y) {
            unsafe { Display::set_pixel(&mut self.framebuffer, point.x, point.y, color.r(), color.g(), color.b()) };
        }
    }
}

impl OriginDimensions for Display {
    fn size(&self) -> Size {
        let info = self.framebuffer.info();
        Size::new(info.width as u32, info.height as u32)
    }
}

impl embedded_graphics::draw_target::DrawTarget for Display {
    type Color = embedded_graphics::pixelcolor::Rgb888;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = embedded_graphics::prelude::Pixel<Self::Color>>
    {
        for pixel in pixels.into_iter() {
            self.draw_pixel(pixel);
        }
        Ok(())
    }
}
