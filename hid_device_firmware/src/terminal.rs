use embedded_graphics::mono_font::{ascii::FONT_6X12, MonoTextStyle};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyleBuilder, Rectangle};
use embedded_graphics::text::{Baseline, Text};
use wio_terminal as bsp;

// Copy from <https://github.com/atsamd-rs/atsamd/blob/master/boards/wio_terminal/examples/microphone.rs>
/// Handly helper for logging text to the screen.
pub struct Terminal<'a> {
    text_style: MonoTextStyle<'a, Rgb565>,
    cursor: Point,
    display: bsp::LCD,
}

impl<'a> Terminal<'a> {
    pub fn new(mut display: bsp::LCD) -> Self {
        // Clear the screen.
        let style = PrimitiveStyleBuilder::new()
            .fill_color(Rgb565::BLACK)
            .build();
        let backdrop =
            Rectangle::with_corners(Point::new(0, 0), Point::new(320, 320)).into_styled(style);
        backdrop.draw(&mut display).ok().unwrap();

        Self {
            text_style: MonoTextStyle::new(&FONT_6X12, Rgb565::WHITE),
            cursor: Point::new(0, 0),
            display,
        }
    }

    pub fn clear(&mut self) {
        self.display.clear(Rgb565::BLACK).ok().unwrap();
        self.cursor = Point::new(0, 0);
    }

    pub fn write_str(&mut self, str: &str) {
        for character in str.chars() {
            self.write_character(character);
        }
    }

    pub fn write_character(&mut self, c: char) {
        if self.cursor.x >= 320 || c == '\n' {
            self.cursor = Point::new(0, self.cursor.y + FONT_6X12.character_size.height as i32);
        }
        if self.cursor.y >= 240 {
            // Clear the screen.
            let style = PrimitiveStyleBuilder::new()
                .fill_color(Rgb565::BLACK)
                .build();
            let backdrop =
                Rectangle::with_corners(Point::new(0, 0), Point::new(320, 320)).into_styled(style);
            backdrop.draw(&mut self.display).ok().unwrap();
            self.cursor = Point::new(0, 0);
        }

        if c != '\n' {
            let mut buf = [0u8; 8];
            Text::with_baseline(
                c.encode_utf8(&mut buf),
                self.cursor,
                self.text_style,
                Baseline::Top,
            )
            .draw(&mut self.display)
            .ok()
            .unwrap();

            self.cursor.x += (FONT_6X12.character_size.width + FONT_6X12.character_spacing) as i32;
        }
    }
}
