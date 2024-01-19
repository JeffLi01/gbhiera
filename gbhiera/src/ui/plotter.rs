use plotters::{prelude::*, style::full_palette::{GREY_100, GREY_300, GREY_600, GREY_800}};
use slint::SharedPixelBuffer;

use bhiera::{View, DataProvider, Bhiera, Model};

#[derive(Clone)]
struct PlotStyle {
    bg: RGBColor,
    fg: RGBColor,
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Plotter<'a> {
    width: u32,
    char_width: u32,
    char_height: u32,
    hex_byte_width: u32,
    offset_view_width: u32,
    hex_view_width: u32,
    char_view_width: u32,
    offset_view: PlotStyle,
    hex_view: PlotStyle,
    char_view: PlotStyle,
    style: TextStyle<'a>,
}

impl<'a> Plotter<'a> {
    pub fn with_font(typeface: &'a str, size: f64) -> Self {
        let mut buf: Vec<_> = vec![0; 3];
        let backend = BitMapBackend::with_buffer(&mut buf, (1, 1));
        let style = TextStyle::from((typeface, size).into_font()).color(&BLACK);
        let (char_width, char_height): (u32, u32) = backend.estimate_text_size("C", &style).unwrap();
        let (hex_width, _): (u32, u32) = backend.estimate_text_size("HH", &style).unwrap();
        let (offset_view_width, _): (u32, u32) = backend.estimate_text_size("00000000 ", &style).unwrap();
        let hex_view_width = (char_width + hex_width) * 16 + char_width * 2;
        let char_view_width = char_width * 16;
        let img_width = {
            let right_margin = char_width;
            offset_view_width + hex_view_width + char_view_width + right_margin
        };

        Self {
            width: img_width,
            char_width,
            char_height,
            hex_byte_width: hex_width,
            offset_view_width,
            hex_view_width,
            char_view_width,
            offset_view: PlotStyle { bg: GREY_300, fg: GREY_600 },
            hex_view: PlotStyle { bg: WHITE, fg: BLACK },
            char_view: PlotStyle { bg: GREY_100, fg: GREY_800 },
            style,
        }
    }

    pub fn geometry<D: DataProvider + 'static>(&self, provider: &D) -> (u32, u32) {
        let total_line_count = (provider.len() + 15) / 16;
        let height = self.char_height * total_line_count as u32;
        (self.width, height)
    }

    pub fn plot(&self, bhiera: &Bhiera, view_start: i32, view_height: i32) -> slint::Image {
        let start_line = (view_start + self.char_height as i32 - 1) / self.char_height as i32;
        let line_count = view_height as u32 / self.char_height;
        let view = bhiera.get_view(start_line as usize * 16, line_count as usize * 16);
        match view {
            Some(view) => self.do_plot(view_height, view),
            None => slint::Image::default(),
        }
    }

    fn do_plot(&self, img_height: i32, view: View) -> slint::Image {
        let mut pixel_buffer = SharedPixelBuffer::new(self.width, img_height as u32);
        let size = (pixel_buffer.width(), pixel_buffer.height());
        let mut backend = BitMapBackend::with_buffer(pixel_buffer.make_mut_bytes(), size);
        let (width, height) = backend.get_size();
    
        backend.draw_rect((0, 0), (width as i32, height as i32), &self.hex_view.bg, true).unwrap();
        backend.draw_rect((0, 0), (self.offset_view_width as i32, height as i32), &self.offset_view.bg, true).unwrap();
    
        let style = self.style.color(&self.offset_view.fg);
        for (line, line_offset) in (0..view.size()).step_by(16).enumerate() {
            let offset = format!("{:08X}", line_offset);
            backend.draw_text(&offset, &style, (0, (line * self.char_height as usize) as i32)).unwrap();
        }
    
        let style = style.color(&self.hex_view.fg);
        for i in 0..view.size() {
            let line = i / 16;
            let byte_hex = format!("{:02X}", view.byte(i));
            let index = i % 16;
            let mut x = self.offset_view_width + (self.char_width + self.hex_byte_width) * index as u32;
            if index >= 8 {
                x += self.char_width;
            }
            let y = line as u32 * self.char_height;
            backend.draw_text(&byte_hex, &style, (x as i32, y as i32)).unwrap();
        }
    
        let style = style.color(&self.char_view.fg);
        for i in 0..view.size() {
            let line = i / 16;
            let byte_char = {
                let c = match char::from_u32(view.byte(i) as u32) {
                    Some(c) => if c.is_ascii_graphic() { c } else { '.' },
                    None => '.',
                };
                format!("{}", c)
            };
            let x = self.offset_view_width + self.hex_view_width + (i as u32 % 16) * self.char_width;
            let y = line as u32 * self.char_height;
            backend.draw_text(&byte_char, &style, (x as i32, y as i32)).unwrap();
        }
    
        backend.present().unwrap();
        drop(backend);
        slint::Image::from_rgb8(pixel_buffer)
    }
}
