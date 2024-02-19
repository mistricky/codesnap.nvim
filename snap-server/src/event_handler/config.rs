use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct Config {
    breadcrumbs: bool,
    column_number: bool,
    mac_window_bar: bool,
    opacity: bool,
    watermark: Option<String>,
    auto_load: bool,
}

impl From<&str> for Config {
    fn from(value: &str) -> Self {
        return serde_json::from_str(value).unwrap();
    }
}
