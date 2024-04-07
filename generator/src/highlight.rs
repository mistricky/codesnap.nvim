use cosmic_text::{Attrs, Family, Style, Weight};
use syntect::{
    easy::HighlightLines,
    highlighting::{FontStyle, ThemeSet},
    parsing::{SyntaxReference, SyntaxSet},
    util::LinesWithEndings,
};

use crate::components::interface::render_error::RenderError;

pub struct Highlight {
    content: String,
    code_file_path: String,
    extension: Option<String>,
    font_family: String,
}

pub type HighlightResult<'a> = Vec<(&'a str, Attrs<'a>)>;

impl Highlight {
    pub fn new(
        content: String,
        font_family: String,
        code_file_path: String,
        extension: Option<String>,
    ) -> Highlight {
        Highlight {
            content,
            code_file_path,
            extension,
            font_family,
        }
    }

    fn guess_syntax(&self, syntax_set: &SyntaxSet) -> Result<SyntaxReference, RenderError> {
        // The extension exist only when users specified explicitly
        // By default, using filepath to detect what syntax should use
        let syntax = match &self.extension {
            Some(extension) => syntax_set
                .find_syntax_by_extension(&extension)
                .ok_or(RenderError::HighlightCodeFailed(extension.to_string()))?,
            None => syntax_set
                .find_syntax_for_file(&self.code_file_path)
                .map_err(|_| RenderError::NoSuchFile(self.code_file_path.to_string()))?
                .ok_or(RenderError::HighlightCodeFailed(
                    self.code_file_path.to_string(),
                ))?,
        };

        Ok(syntax.to_owned())
    }

    pub fn parse(
        &self,
        theme_folder: &str,
        theme: &str,
    ) -> Result<Vec<(&str, Attrs)>, RenderError> {
        let syntax_set = two_face::syntax::extra_newlines();
        let theme_set = ThemeSet::load_from_folder(theme_folder)
            .map_err(|_| RenderError::HighlightThemeLoadFailed)?;
        let syntax = &self.guess_syntax(&syntax_set)?;
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
