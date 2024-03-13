use cosmic_text::{Attrs, Color, Family};

use crate::{code::calc_wh, text::render_line};

use super::component::Component;

pub struct Watermark {
    value: Option<String>,
    margin_bottom: f32,
    font_family: String,
    children: Vec<Box<dyn Component>>,
}

impl Component for Watermark {
    fn draw_self(
        &self,
        pixmap: &mut tiny_skia::Pixmap,
        context: &super::component::ComponentContext,
    ) {
        if let Some(value) = &self.value {
            let (_, height) = calc_wh(&value, 9., 20.);
            let attrs = Attrs::new()
                .family(Family::Name(&self.font_family))
                .color(Color::rgba(255, 255, 255, 27));
            let y = pixmap.height() as f32 - self.margin_bottom - height;

            render_line(
                0.,
                y / context.scale_factor,
                pixmap.width() as f32,
                pixmap.height() as f32,
                20.,
                20.,
                context.scale_factor,
                value,
                attrs,
                Color::rgba(255, 255, 255, 127),
                pixmap,
            );
        }
    }

    fn get_children(&self) -> &Vec<Box<dyn Component>> {
        &self.children
    }
}

impl Watermark {
    pub fn new(value: Option<String>, font_family: String, margin_bottom: f32) -> Watermark {
        Watermark {
            value,
            font_family,
            children: vec![],
            margin_bottom,
        }
    }
}
