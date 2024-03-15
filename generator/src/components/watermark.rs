use cosmic_text::{Align, Attrs, Family};

use crate::{code::calc_wh, text::FontRenderer};

use super::component::{Component, ParentComponent};

pub struct Watermark {
    value: Option<String>,
    margin_bottom: f32,
    font_family: String,
    children: Vec<Box<dyn Component>>,
}

impl Component for Watermark {
    fn draw_self(
        &self,
        _: ParentComponent,
        pixmap: &mut tiny_skia::Pixmap,
        context: &super::component::ComponentContext,
    ) {
        if let Some(value) = &self.value {
            let (_, height) = calc_wh(&value, 9., 20.);
            let attrs = Attrs::new().family(Family::Name(&self.font_family));
            let y = pixmap.height() as f32 - self.margin_bottom - height;

            FontRenderer::new(20., 20., context.scale_factor).draw_line(
                0.,
                y / context.scale_factor,
                pixmap.width() as f32,
                pixmap.height() as f32,
                value,
                attrs,
                Some(Align::Center),
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
