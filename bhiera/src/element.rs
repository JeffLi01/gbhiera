pub enum Element {
    Byte {
        text: String,
        x: i32,
        y: i32,
        fg: (u8, u8, u8),
    },
    Rectangle {
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        bg: (u8, u8, u8),
    },
    Line {
        from_x: i32,
        from_y: i32,
        to_x: i32,
        to_y: i32,
        color: (u8, u8, u8),
        width: u32,
    },
}

impl Element {
    pub fn byte(text: String, x: i32, y: i32, fg: (u8, u8, u8)) -> Element {
        Element::Byte { text, x, y, fg }
    }

    pub fn rectangle(x: i32, y: i32, width: i32, height: i32, bg: (u8, u8, u8)) -> Element {
        Element::Rectangle {
            x,
            y,
            width,
            height,
            bg,
        }
    }

    pub fn line(
        from_x: i32,
        from_y: i32,
        to_x: i32,
        to_y: i32,
        color: (u8, u8, u8),
        width: u32,
    ) -> Element {
        Element::Line {
            from_x,
            from_y,
            to_x,
            to_y,
            color,
            width,
        }
    }
}
