pub struct TextElement {
    pub text: String,
    pub x: i32,
    pub y: i32,
    pub fg: (u8, u8, u8),
}

pub struct RectangleElement {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub bg: (u8, u8, u8),
}

pub struct LineElement {
    pub from_x: i32,
    pub from_y: i32,
    pub to_x: i32,
    pub to_y: i32,
    pub color: (u8, u8, u8),
    pub width: u32,
}

pub enum Element {
    Byte(TextElement),
    Rectangle(RectangleElement),
    Line(LineElement),
}
