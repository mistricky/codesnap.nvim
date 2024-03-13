use cosmic_text::{Attrs, Family, Style, Weight};
use syntect::{
    easy::HighlightLines,
    highlighting::{FontStyle, ThemeSet},
    parsing::SyntaxSet,
    util::LinesWithEndings,
};

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

    pub fn parse(&self, theme: &str) -> Vec<(&str, Attrs)> {
        let syntax_set = SyntaxSet::load_defaults_newlines();
        let theme_set = ThemeSet::load_defaults();
        let syntax = match &self.extension {
            Some(extension) => syntax_set.find_syntax_by_extension(extension),
            None => {
                syntax_set.find_syntax_by_name(&self.language.clone().unwrap_or("Rust".to_string()))
            }
        }
        .unwrap();

        let mut h = HighlightLines::new(syntax, &theme_set.themes[theme]);
        let attrs = Attrs::new().family(Family::Name(self.font_family.as_ref()));

        LinesWithEndings::from(&self.content)
            .map(|line| h.highlight_line(line, &syntax_set).unwrap())
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
            .collect::<HighlightResult>()
    }
}
