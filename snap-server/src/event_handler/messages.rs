#[derive(PartialEq, Hash)]
pub enum Message {
    PreviewCode,
    ConfigSetup,
    Copy,
    OpenPreview,
    Unknown,
}

impl Eq for Message {}

impl From<String> for Message {
    fn from(value: String) -> Self {
        match value.as_str() {
            "open_preview" => Message::OpenPreview,
            "copy" => Message::Copy,
            "preview_code" => Message::PreviewCode,
            "config_setup" => Message::ConfigSetup,
            _ => Message::Unknown,
        }
    }
}
