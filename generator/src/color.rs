use std::i64;
use tiny_skia::Color;

const HEX_COLOR_LENGTH: usize = 7;
const HEX_COLOR_WITH_ALPHA_LENGTH: usize = 9;

pub struct RgbaColor {
    pub color: Color,
}

pub fn is_valid_hex_color(color: &str) -> bool {
    (color.len() == HEX_COLOR_LENGTH || color.len() == HEX_COLOR_WITH_ALPHA_LENGTH)
        && color.starts_with("#")
}

fn parse_color_to_rgba_hex(hex: &str) -> String {
    if !is_valid_hex_color(&hex) || hex.len() == HEX_COLOR_WITH_ALPHA_LENGTH {
        hex.to_string()
    } else {
        format!("{}ff", hex)
    }
}

impl Into<RgbaColor> for String {
    fn into(self) -> RgbaColor {
        let rgba_hex_color = parse_color_to_rgba_hex(&self);
        // Remove the '#' symbol
        let hex_color = &rgba_hex_color.to_lowercase()[1..rgba_hex_color.len()];
        let chars = hex_color.chars().collect::<Vec<char>>();
        let splits = &chars
            .chunks(2)
            .map(|chunk| i64::from_str_radix(&chunk.iter().collect::<String>(), 16).unwrap())
            .collect::<Vec<_>>();

        RgbaColor {
            color: Color::from_rgba8(
                splits[0] as u8,
                splits[1] as u8,
                splits[2] as u8,
                splits[3] as u8,
            ),
        }
    }
}
