use cosmic_text::{Attrs, Color, Family};

use crate::{code::calc_wh, text::render_rich_text};

use super::component::Component;

pub struct Text {
    x: f32,
    y: f32,
    text: String,
    width: f32,
    height: f32,
    children: Vec<Box<dyn Component>>,
    font_family: String,
    font_size: f32,
    line_height: f32,
}

impl Component for Text {
    fn draw_self(
        &self,
        pixmap: &mut tiny_skia::Pixmap,
        context: &super::component::ComponentContext,
    ) {
        let attrs = Attrs::new().family(Family::Name(self.font_family.as_ref()));

        render_rich_text(
            self.x,
            self.y,
            self.width,
            self.height,
            self.font_size,
            self.line_height,
            context.scale_factor,
            vec![(&self.text, attrs)],
            Color::rgb(0xFF, 0xFF, 0xFF),
            pixmap,
        );
    }

    fn get_children(&self) -> &Vec<Box<dyn Component>> {
        &self.children
    }
}

impl Text {
    pub fn new(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        text: String,
        font_family: String,
        font_size: f32,
        line_height: f32,
    ) -> Text {
        Text {
            x,
            y,
            width,
            height,
            text,
            font_family,
            font_size,
            line_height,
            children: vec![],
        }
    }
}
