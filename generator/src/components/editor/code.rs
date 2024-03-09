use cosmic_text::{Attrs, Buffer, Color, Family, FontSystem, Metrics, Shaping, SwashCache};
use tiny_skia::{Paint, Rect, Transform};

use crate::{
    components::component::{Component, ComponentContext},
    highlight::Highlight,
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
        let ComponentContext { scale_factor } = context;
        let highlight = Highlight::new(
            self.value.clone(),
            self.font_family.clone(),
            self.language.clone(),
            self.extension.clone(),
        );
        let highlight_result = highlight.parse(&self.theme);
        let mut font_system = FontSystem::new();
        let mut swash_cache = SwashCache::new();
        let metrics = Metrics::new(self.font_size, self.line_height).scale(scale_factor.clone());
        let mut buffer = Buffer::new(&mut font_system, metrics);
        let mut buffer = buffer.borrow_with(&mut font_system);
        let attrs = Attrs::new();

        buffer.set_size(self.w * scale_factor, self.h * scale_factor);
        buffer.set_rich_text(highlight_result, attrs, Shaping::Advanced);
        buffer.draw(
            &mut swash_cache,
            self.default_font_color,
            |x, y, w, h, color| {
                let mut paint = Paint {
                    anti_alias: true,
                    ..Default::default()
                };

                paint.set_color_rgba8(color.r(), color.g(), color.b(), color.a());
                let rect = Rect::from_xywh(
                    x as f32 + self.x * scale_factor,
                    y as f32 + self.y * scale_factor,
                    w as f32,
                    h as f32,
                )
                .unwrap();
                pixmap.fill_rect(rect, &paint, Transform::identity(), None);
            },
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

    pub fn theme(mut self, theme: String) -> Self {
        self.theme = theme;
        self
    }

    pub fn width(mut self, w: f32) -> Self {
        self.w = w;
        self
    }

    pub fn height(mut self, h: f32) -> Self {
        self.h = h;
        self
    }

    pub fn line_height(mut self, line_height: f32) -> Self {
        self.line_height = line_height;
        self
    }

    pub fn font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }

    pub fn default_font_color(mut self, default_font_color: Color) -> Self {
        self.default_font_color = default_font_color;
        self
    }
}
