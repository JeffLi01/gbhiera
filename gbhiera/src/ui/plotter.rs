use image::{ImageBuffer, Pixel, Rgb};
use plotters::prelude::*;
use slint::SharedPixelBuffer;

use bhiera::{Bhiera, DataProvider, Element, Geometry, Model};

#[allow(dead_code)]
#[derive(Clone)]
pub struct Plotter<'a> {
    pub config: Geometry,
    text_style: TextStyle<'a>,
}

impl<'a> Plotter<'a> {
    pub fn with_font(typeface: &'a str, size: f64) -> Self {
        let mut buf: Vec<_> = vec![0; 3];
        let backend = BitMapBackend::with_buffer(&mut buf, (1, 1));
        let text_style = TextStyle::from((typeface, size).into_font()).color(&BLACK);
        let (char_width, char_height): (u32, u32) =
            backend.estimate_text_size("C", &text_style).unwrap();
        let (hex_byte_width, _): (u32, u32) =
            backend.estimate_text_size("HH", &text_style).unwrap();
        let (offset_view_width, _): (u32, u32) = backend
            .estimate_text_size("00000000 ", &text_style)
            .unwrap();

        Self {
            config: Geometry::new(char_width, char_height, hex_byte_width, offset_view_width),
            text_style,
        }
    }

    pub fn geometry<D: DataProvider + 'static>(&self, provider: &D) -> (u32, u32) {
        (self.config.width(), self.config.height(provider.len()))
    }

    pub fn plot(&self, bhiera: &Bhiera, view_start: i32, view_height: i32) -> slint::Image {
        let view = bhiera.get_view(view_start as u32, view_height as u32);
        match view {
            Some(view) => {
                let cursors = view.get_cursur();
                let mut pixel_buffer =
                    SharedPixelBuffer::new(self.config.width(), view_height as u32);
                let size = (pixel_buffer.width(), pixel_buffer.height());
                let mut backend = BitMapBackend::with_buffer(pixel_buffer.make_mut_bytes(), size);

                let mut style;
                let mut bg_color;
                let mut fg_color;
                for element in view {
                    match element {
                        Element::Byte(text) => {
                            fg_color = RGBColor(text.fg.0, text.fg.1, text.fg.2);
                            style = self.text_style.color(&fg_color);
                            backend
                                .draw_text(&text.text, &style, (text.x, text.y))
                                .unwrap();
                        }
                        Element::Rectangle(rectangle) => {
                            bg_color = RGBColor(rectangle.bg.0, rectangle.bg.1, rectangle.bg.2);
                            backend
                                .draw_rect(
                                    (rectangle.x, rectangle.y),
                                    (rectangle.x + rectangle.width, rectangle.y + rectangle.height),
                                    &bg_color,
                                    true,
                                )
                                .unwrap();
                        }
                    };
                }

                backend.present().unwrap();
                drop(backend);

                for (x, y, w, h) in cursors {
                    let (width, height) = (pixel_buffer.width(), pixel_buffer.height());
                    let mut buffer = ImageBuffer::<Rgb<u8>, _>::from_raw(
                        width,
                        height,
                        pixel_buffer.make_mut_bytes(),
                    )
                    .unwrap();
                    for row in x..(x + w) {
                        for col in y..(y + h) {
                            if (row < width) && (col < height) {
                                let p = buffer.get_pixel_mut(row, col);
                                p.invert();
                            }
                        }
                    }
                }
                slint::Image::from_rgb8(pixel_buffer)
            }
            None => slint::Image::default(),
        }
    }
}
