#![no_std]
#![no_main]

mod framebuffer;

use core::panic::PanicInfo;
use bootloader_api::info::Optional;
use framebuffer::Display;

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{
        Circle, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StrokeAlignment, Triangle,
    },
    text::{Alignment, Text},
};

bootloader_api::entry_point!(main);

fn main(bootinfo: &'static mut bootloader_api::BootInfo) -> ! {
    if let Optional::Some(ref mut framebuffer) = bootinfo.framebuffer {
        let mut display = Display::new(framebuffer);
        display.clear();
        // Create styles used by the drawing operations.
    
        let thin_stroke = PrimitiveStyle::with_stroke(Rgb888::CSS_CYAN, 1);
        let thick_stroke = PrimitiveStyle::with_stroke(Rgb888::CSS_CYAN, 3);
        let border_stroke = PrimitiveStyleBuilder::new()
            .stroke_color(Rgb888::CSS_CYAN)
            .stroke_width(3)
            .stroke_alignment(StrokeAlignment::Inside)
            .build();
        let fill = PrimitiveStyle::with_fill(Rgb888::new(0, 20, 40));
        let character_style = MonoTextStyle::new(&FONT_6X10, Rgb888::CSS_CYAN);

        let yoffset = 10;

        display.bounding_box().into_styled(fill).draw(&mut display).unwrap();

        // Draw a 3px wide outline around the display.
        display
            .bounding_box()
            .into_styled(border_stroke)
            .draw(&mut display)
            .unwrap();

        // Draw a triangle.
        Triangle::new(
            Point::new(16, 16 + yoffset),
            Point::new(16 + 16, 16 + yoffset),
            Point::new(16 + 8, yoffset),
        )
        .into_styled(thin_stroke)
        .draw(&mut display)
        .unwrap();

        // Draw a filled square
        Rectangle::new(Point::new(52, yoffset), Size::new(16, 16))
            .into_styled(fill)
            .draw(&mut display)
            .unwrap();

        // Draw a circle with a 3px wide stroke.
        Circle::new(Point::new(88, yoffset), 17)
            .into_styled(thick_stroke)
            .draw(&mut display)
            .unwrap();

        // Draw centered text.
        let text = "embedded-graphics";
        Text::with_alignment(
            text,
            display.bounding_box().center() + Point::new(0, 15),
            character_style,
            Alignment::Center,
        )
        .draw(&mut display)
        .unwrap();
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}