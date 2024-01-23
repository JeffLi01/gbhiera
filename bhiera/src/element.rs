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

pub enum Element {
    Byte(TextElement),
    Rectangle(RectangleElement),
}
