use cosmic_text::{Attrs, Family, Style, Weight};
use syntect::{
    easy::HighlightLines,
    highlighting::{FontStyle, ThemeSet},
    parsing::SyntaxSet,
    util::LinesWithEndings,
};

use crate::components::interface::render_error::RenderError;

pub struct Highlight {
    content: String,
    language: Option<String>,
    extension: Option<String>,
    font_family: String,
}

pub type HighlightResult<'a> = Vec<(&'a str, Attrs<'a>)>;

impl Highlight {
    pub fn new(
        content: String,
        font_family: String,
        language: Option<String>,
        extension: Option<String>,
    ) -> Highlight {
        Highlight {
            content,
            language,
            extension,
            font_family,
        }
    }

    pub fn parse(
        &self,
        theme_folder: &str,
        theme: &str,
    ) -> Result<Vec<(&str, Attrs)>, RenderError> {
        let syntax_set = SyntaxSet::load_defaults_newlines();
        let theme_set = ThemeSet::load_from_folder(theme_folder)
            .map_err(|_| RenderError::HighlightThemeLoadFailed)?;
        let syntax = match &self.extension {
            Some(extension) => syntax_set
                .find_syntax_by_extension(extension)
                .ok_or(RenderError::HighlightCodeFailed(extension.to_owned())),
            None => {
                let language = &self.language.clone().unwrap_or("Rust".to_string());

                syntax_set
                    .find_syntax_by_name(language)
                    .ok_or(RenderError::HighlightCodeFailed(language.to_owned()))
            }
        }?;

        let mut highlight = HighlightLines::new(syntax, &theme_set.themes[theme]);
        let attrs = Attrs::new().family(Family::Name(self.font_family.as_ref()));

        Ok(LinesWithEndings::from(&self.content)
            .map(|line| highlight.highlight_line(line, &syntax_set).unwrap())
            .fold(vec![], |acc, cur| [acc, cur].concat())
            .into_iter()
            .map(move |(style, str)| {
                let syntect::highlighting::Color { r, g, b, a: _ } = style.foreground;
                let attrs = match style.font_style {
                    FontStyle::BOLD => attrs.weight(Weight::BOLD),
                    FontStyle::ITALIC => attrs.style(Style::Italic),
                    FontStyle::UNDERLINE => attrs.style(Style::Normal),
                    _ => attrs,
                };

                (str, attrs.color(cosmic_text::Color::rgb(r, g, b)))
            })
            .collect::<HighlightResult>())
    }
}
