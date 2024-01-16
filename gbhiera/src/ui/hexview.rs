use plotters::{prelude::*, style::full_palette::{GREY_100, GREY_300, GREY_600}};
use rgb::RGB;
use slint::SharedPixelBuffer;

struct PlotStyle {
    bg: RGBColor,
    fg: RGBColor,
}

struct PlotConfig<'a> {
    width: u32,
    height: u32,
    char_width: u32,
    char_height: u32,
    hex_width: u32,
    offset_width: u32,
    hex_view_width: u32,
    char_view_width: u32,
    offset: PlotStyle,
    hex: PlotStyle,
    char: PlotStyle,
    style: TextStyle<'a>,
}

fn setup(img_height: u32) -> PlotConfig<'static> {
    let mut buf: Vec<_> = vec![0; 3];
    let backend = BitMapBackend::with_buffer(&mut buf, (1, 1));
    let style = TextStyle::from(("Courier New", 18).into_font()).color(&BLACK);
    let (char_width, char_height): (u32, u32) = backend.estimate_text_size("C", &style).unwrap();
    let (hex_width, _): (u32, u32) = backend.estimate_text_size("HH", &style).unwrap();
    let (offset_width, _): (u32, u32) = backend.estimate_text_size("00000000", &style).unwrap();
    // println!("offset_width: {}, hex_width: {}, char_width: {}", offset_width, hex_width, char_width);
    drop(backend);
    let hex_view_width = char_width * 16 + hex_width * 16 + char_width * 3;
    let char_view_width = char_width * 16;
    let img_width = offset_width + hex_view_width + char_view_width + char_width;

    PlotConfig {
        width: img_width,
        height: img_height,
        char_width,
        char_height,
        hex_width,
        offset_width,
        hex_view_width,
        char_view_width,
        offset: PlotStyle { bg: GREY_300, fg: GREY_600 },
        hex: PlotStyle { bg: WHITE, fg: BLACK },
        char: PlotStyle { bg: GREY_100, fg: BLACK },
        style,
    }
}

fn pre_do_plot(config: &PlotConfig, pixel_buffer: &mut SharedPixelBuffer<RGB<u8>>) {
    let size = (pixel_buffer.width(), pixel_buffer.height());
    let mut backend = BitMapBackend::with_buffer(pixel_buffer.make_mut_bytes(), size);
    let (width, height) = backend.get_size();

    backend.draw_rect((0, 0), (width as i32, height as i32), &config.hex.bg, true).unwrap();
    backend.draw_rect((0, 0), (config.offset_width as i32, height as i32), &config.offset.bg, true).unwrap();
    backend.draw_rect(((config.offset_width + config.hex_view_width) as i32, 0), ((config.offset_width + config.hex_view_width + config.char_view_width) as i32, height as i32), &config.char.bg, true).unwrap();

    backend.present().unwrap();
    drop(backend);
}

fn do_plot(config: &PlotConfig, start_line: usize, bytes: Vec<u8>, pixel_buffer: &mut SharedPixelBuffer<RGB<u8>>) -> Result<(), Box<dyn std::error::Error>> {
    let size = (pixel_buffer.width(), pixel_buffer.height());
    let mut backend = BitMapBackend::with_buffer(pixel_buffer.make_mut_bytes(), size);

    let offset_style = &config.style;
    let line_count = (bytes.len() + 15) / 16;
    for line in 0..(line_count) {
        let offset = format!("{:08X}", (start_line + line) << 4);
        backend.draw_text(&offset, offset_style, (0, (line * config.char_height as usize) as i32)).unwrap();
    }
    for (i, byte) in bytes.iter().enumerate() {
        let line = i / 16;
        let byte_hex = format!("{:02X}", byte);
        let index = i % 16;
        let x = if index < 8 {
            config.offset_width + config.char_width + (config.char_width + config.hex_width) * index as u32
        } else {
            config.offset_width + config.char_width * 2 + (config.char_width + config.hex_width) * index as u32
        } as i32;
        backend.draw_text(&byte_hex, &config.style, (x, (line as u32 * config.char_height) as i32)).unwrap();
        let byte_char = {
            let c = match char::from_u32(*byte as u32) {
                Some(c) => if c.is_ascii_graphic() {
                    c
                } else {
                    '.'
                },
                None => '.',
            };
            format!("{}", c)
        };
        backend.draw_text(&byte_char, &config.style, ((config.offset_width + config.hex_view_width + (i as u32 % 16) * config.char_width) as i32, (line as u32 * config.char_height) as i32)).unwrap();
    }

    backend.present().unwrap();
    drop(backend);
    Ok(())
}

pub fn render_plot(start_line: i32, img_height: i32, bytes: Vec<u8>) -> slint::Image {
    println!("render_plot");
    let config = setup(img_height as u32);
    let mut pixel_buffer = SharedPixelBuffer::new(config.width, config.height);
    pre_do_plot(&config, &mut pixel_buffer);
    do_plot(&config, start_line as usize, bytes, &mut pixel_buffer).unwrap();
    slint::Image::from_rgb8(pixel_buffer)
}
