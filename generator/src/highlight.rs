use std::collections::HashMap;

use cosmic_text::{Attrs, Family, Style, Weight};
use syntect::{
    easy::HighlightLines,
    highlighting::{FontStyle, ThemeSet},
    parsing::{SyntaxReference, SyntaxSet},
    util::LinesWithEndings,
};

use crate::components::interface::render_error::RenderError;

type SourceMap = HashMap<&'static str, &'static str>;

pub struct Highlight {
    content: String,
    code_file_path: String,
    extension: Option<String>,
    font_family: String,
    highlighting_language_source_map: SourceMap,
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
            highlighting_language_source_map: HashMap::from([("PHP", "<?php")]),
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

        // The Syntect clearly distinguish between PHP and PHP Source
        // Should use PHP as highlight language if the source content contains "<php" tag
        // Should use PHP Source as highlight language if the source content not contains "<php" tag
        if let Some(identifier) = self.highlighting_language_source_map.get(&syntax.name[..]) {
            if !self.content.contains(identifier) {
                return Ok(syntax_set
                    .find_syntax_by_name(&format!("{} Source", &syntax.name))
                    .unwrap_or(syntax)
                    .to_owned());
            }
        }

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

        // Highlight the content line by line using highlight_line function
        Ok(LinesWithEndings::from(&self.content)
            .map(|line| {
                highlight
                    .highlight_line(line, &syntax_set)
                    .unwrap()
                    .into_iter()
                    .map(|(style, str)| {
                        let syntect::highlighting::Color { r, g, b, a: _ } = style.foreground;
                        let attrs = match style.font_style {
                            FontStyle::BOLD => attrs.weight(Weight::BOLD),
                            FontStyle::ITALIC => attrs.style(Style::Italic),
                            FontStyle::UNDERLINE => attrs.style(Style::Normal),
                            _ => attrs,
                        };

                        (str, attrs.color(cosmic_text::Color::rgb(r, g, b)))
                    })
                    .collect::<HighlightResult>()
            })
            .fold(vec![], |acc, cur| [acc, cur].concat())
            .into_iter()
            .collect::<HighlightResult>())
    }
}
