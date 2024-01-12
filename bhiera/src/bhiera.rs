use crate::DataProvider;

pub struct Bhiera {
    data_provider: Option<Box<dyn DataProvider>>,
}

impl Bhiera {
    pub fn new() -> Self {
        Self {
            data_provider: None,
        }
    }
}

pub trait View {
    fn set_data_provider(&mut self, provider: impl DataProvider + 'static);
    fn get_line(&mut self, line: i32) -> Option<String>;
}

impl View for Bhiera {
    fn set_data_provider(&mut self, provider: impl DataProvider + 'static) {
        self.data_provider.replace(Box::new(provider));
    }
    fn get_line(&mut self, line: i32) -> Option<String> {
        if let Some(binary_data) = &mut self.data_provider {
            let bytes = (*binary_data).get(line as usize * 16, 16);
            if bytes.is_empty() {
                return None;
            }
            let mut s = format!("{:08X} ", line * 16);
            for i in 0..bytes.len() {
                if i < 8 {
                    s.push_str(&format!("{:02X} ", bytes[i]));
                } else {
                    s.push_str(&format!(" {:02X}", bytes[i]));
                }
            }
            s.push_str(&format!("{:width$}", "", width = (16 - bytes.len()) * 3));
            s.push_str("  ");
            for b in bytes {
                let c = match char::from_u32(b as u32) {
                    Some(c) => {
                        if c.is_ascii_graphic() {
                            c
                        } else {
                            '.'
                        }
                    }
                    None => '.',
                };
                s.push(c);
            }
            return Some(s);
        }
        None
    }
}
