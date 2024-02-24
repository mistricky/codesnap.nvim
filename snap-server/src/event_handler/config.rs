use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct Config {
    mac_window_bar: bool,
    opacity: bool,
    watermark: Option<String>,
    auto_load: bool,
    preview_title: String,
    watermark_font_family: String,
    editor_font_family: String,
    highlight_theme: String,
}

impl From<&str> for Config {
    fn from(value: &str) -> Self {
        return serde_json::from_str(value).unwrap();
    }
}
