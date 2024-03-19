use std::i64;
use tiny_skia::Color;

pub struct RgbaColor {
    pub color: Color,
}

impl Into<RgbaColor> for String {
    fn into(self) -> RgbaColor {
        let hex_color = &self.to_lowercase()[1..self.len()];
        let chars = hex_color.chars().collect::<Vec<char>>();
        let splits = &chars
            .chunks(2)
            .map(|chunk| i64::from_str_radix(&chunk.iter().collect::<String>(), 16).unwrap())
            .collect::<Vec<_>>();
        
        let alpha: i64;
        match splits.get(3) {
            Some(x) => alpha = *x,
            None => alpha = 255,
        }

        RgbaColor {
            color: Color::from_rgba8(splits[0] as u8, splits[1] as u8, splits[2] as u8, alpha as u8),
        }
    }
}
