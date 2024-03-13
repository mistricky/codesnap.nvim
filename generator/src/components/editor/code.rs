use cosmic_text::Color;

use crate::{
    components::component::{Component, ComponentContext},
    highlight::Highlight,
    text::render_rich_text,
};

pub struct Code {
    children: Vec<Box<dyn Component>>,
    value: String,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    line_height: f32,
    font_size: f32,
    default_font_color: Color,
    language: Option<String>,
    extension: Option<String>,
    theme: String,
    font_family: String,
}

impl Component for Code {
    fn children(mut self, components: Vec<Box<dyn Component>>) -> Self
    where
        Self: Sized,
    {
        self.children = components;
        self
    }

    fn get_children(&self) -> &Vec<Box<dyn Component>> {
        &self.children
    }

    fn draw_self(&self, pixmap: &mut tiny_skia::Pixmap, context: &ComponentContext) {
        let highlight = Highlight::new(
            self.value.clone(),
            self.font_family.clone(),
            self.language.clone(),
            self.extension.clone(),
        );
        let highlight_result = highlight.parse(&self.theme);

        render_rich_text(
            self.x,
            self.y,
            self.w,
            self.h,
            self.font_size,
            self.line_height,
            context.scale_factor,
            highlight_result,
            self.default_font_color,
            pixmap,
        );
    }
}

impl Code {
    pub fn new(
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        value: String,
        language: Option<String>,
        extension: Option<String>,
        font_family: String,
    ) -> Code {
        Code {
            x,
            y,
            w,
            h,
            value,
            font_family,
            line_height: 15.,
            font_size: 15.,
            children: vec![],
            default_font_color: Color::rgb(0xFF, 0xFF, 0xFF),
            language,
            extension,
            theme: "base16-ocean.dark".to_string(),
        }
    }

    pub fn line_height(mut self, line_height: f32) -> Self {
        self.line_height = line_height;
        self
    }

    pub fn font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }
}
