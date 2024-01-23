#[derive(Clone, Copy, Default)]
pub struct Geometry {
    pub char_width: u32,
    pub char_height: u32,
    pub hex_byte_width: u32,
    pub offset_view_width: u32,
}

impl Geometry {
    pub fn width(&self) -> u32 {
        let right_margin = self.char_width;
        self.offset_view_width + self.hex_view_width() + self.char_view_width() + right_margin
    }

    pub fn hex_view_width(&self) -> u32 {
        (self.char_width + self.hex_byte_width) * 16 + self.char_width * 2
    }

    pub fn char_view_width(&self) -> u32 {
        self.char_width * 16
    }
}
