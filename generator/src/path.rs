use regex::Regex;
use std::env::{var, VarError};

pub fn parse_save_path(path: String) -> Result<String, VarError> {
    let home_path = var("HOME")?;
    let regex = Regex::new(r"(~|$HOME)").unwrap();
    let path = regex.replace_all(&path, home_path);

    Ok(path.to_string())
}
