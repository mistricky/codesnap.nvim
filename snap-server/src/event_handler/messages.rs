#[derive(PartialEq, Hash)]
pub enum Message {
    PreviewCode,
    Unknown,
}

impl Eq for Message {}

impl From<String> for Message {
    fn from(value: String) -> Self {
        match value.as_str() {
            "preview_code" => Message::PreviewCode,
            _ => Message::Unknown,
        }
    }
}
