use nvim_oxi::lua;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, RenderError>;

#[derive(Debug, Error)]
pub enum RenderError {
    #[error("Highlight code failed!")]
    HighlightThemeLoadFailed,

    #[error("Find Highlight theme for {0} failed")]
    HighlightCodeFailed(String),

    #[error("Unable to parse unknown background theme {0}")]
    UnknownBackgroundTheme(String),

    #[error("Invalid hex color {0}")]
    InvalidHexColor(String),
}

impl From<RenderError> for nvim_oxi::Error {
    fn from(err: RenderError) -> Self {
        nvim_oxi::Error::Lua(lua::Error::RuntimeError(err.to_string()))
    }
}
