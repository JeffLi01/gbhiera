use plotters::{prelude::*, style::full_palette::{GREY_100, GREY_300, GREY_600, GREY_800}};
use rgb::RGB;
use slint::SharedPixelBuffer;

use bhiera::View;

#[derive(Clone)]
pub struct PlotStyle {
    bg: RGBColor,
    fg: RGBColor,
}

#[derive(Clone)]
pub struct PlotConfig<'a> {
    pub width: u32,
    char_width: u32,
    pub char_height: u32,
    hex_width: u32,
    offset_width: u32,
    hex_view_width: u32,
    char_view_width: u32,
    offset_view: PlotStyle,
    hex_view: PlotStyle,
    char_view: PlotStyle,
    style: TextStyle<'a>,
}

impl<'a> PlotConfig<'a> {
    pub fn new(typeface: &'a str, size: f64) -> Self {
        let mut buf: Vec<_> = vec![0; 3];
        let backend = BitMapBackend::with_buffer(&mut buf, (1, 1));
        let style = TextStyle::from((typeface, size).into_font()).color(&BLACK);
        let (char_width, char_height): (u32, u32) = backend.estimate_text_size("C", &style).unwrap();
        let (hex_width, _): (u32, u32) = backend.estimate_text_size("HH", &style).unwrap();
        let (offset_width, _): (u32, u32) = backend.estimate_text_size("00000000", &style).unwrap();
        // println!("offset_width: {}, hex_width: {}, char_width: {}", offset_width, hex_width, char_width);
        drop(backend);
        let hex_view_width = char_width * 16 + hex_width * 16 + char_width * 3;
        let char_view_width = char_width * 16;
        let img_width = offset_width + hex_view_width + char_view_width + char_width;

        Self {
            width: img_width,
            char_width,
            char_height,
            hex_width,
            offset_width,
            hex_view_width,
            char_view_width,
            offset_view: PlotStyle { bg: GREY_300, fg: GREY_600 },
            hex_view: PlotStyle { bg: WHITE, fg: BLACK },
            char_view: PlotStyle { bg: GREY_100, fg: GREY_800 },
            style,
        }
    }
}

fn pre_do_plot(config: &PlotConfig, pixel_buffer: &mut SharedPixelBuffer<RGB<u8>>) {
    let size = (pixel_buffer.width(), pixel_buffer.height());
    let mut backend = BitMapBackend::with_buffer(pixel_buffer.make_mut_bytes(), size);
    let (width, height) = backend.get_size();

    backend.draw_rect((0, 0), (width as i32, height as i32), &config.hex_view.bg, true).unwrap();
    backend.draw_rect((0, 0), (config.offset_width as i32, height as i32), &config.offset_view.bg, true).unwrap();
    backend.draw_rect(((config.offset_width + config.hex_view_width) as i32, 0), ((config.offset_width + config.hex_view_width + config.char_view_width) as i32, height as i32), &config.char_view.bg, true).unwrap();

    backend.present().unwrap();
    drop(backend);
}

fn do_plot(config: &PlotConfig, start_line: usize, view: View, pixel_buffer: &mut SharedPixelBuffer<RGB<u8>>) {
    let bytes = &view.bytes;
    let size = (pixel_buffer.width(), pixel_buffer.height());
    let mut backend = BitMapBackend::with_buffer(pixel_buffer.make_mut_bytes(), size);

    let line_count = (bytes.len() + 15) / 16;
    let style = config.style.color(&config.offset_view.fg);
    for line in 0..(line_count) {
        let offset = format!("{:08X}", (start_line + line) << 4);
        backend.draw_text(&offset, &style, (0, (line * config.char_height as usize) as i32)).unwrap();
    }

    let style = style.color(&config.hex_view.fg);
    for (i, byte) in bytes.iter().enumerate() {
        let line = i / 16;
        let byte_hex = format!("{:02X}", byte);
        let index = i % 16;
        let x = if index < 8 {
            config.offset_width + config.char_width + (config.char_width + config.hex_width) * index as u32
        } else {
            config.offset_width + config.char_width * 2 + (config.char_width + config.hex_width) * index as u32
        } as i32;
        backend.draw_text(&byte_hex, &style, (x, (line as u32 * config.char_height) as i32)).unwrap();
    }

    let style = style.color(&config.char_view.fg);
    for (i, byte) in bytes.iter().enumerate() {
        let line = i / 16;
        let byte_char = {
            let c = match char::from_u32(*byte as u32) {
                Some(c) => if c.is_ascii_graphic() { c } else { '.' },
                None => '.',
            };
            format!("{}", c)
        };
        backend.draw_text(&byte_char, &style, ((config.offset_width + config.hex_view_width + (i as u32 % 16) * config.char_width) as i32, (line as u32 * config.char_height) as i32)).unwrap();
    }

    backend.present().unwrap();
    drop(backend);
}

pub fn render_plot(config: &PlotConfig, start_line: i32, img_height: i32, view: View) -> slint::Image {
    let mut pixel_buffer = SharedPixelBuffer::new(config.width, img_height as u32);
    pre_do_plot(config, &mut pixel_buffer);
    do_plot(config, start_line as usize, view, &mut pixel_buffer);
    slint::Image::from_rgb8(pixel_buffer)
}
