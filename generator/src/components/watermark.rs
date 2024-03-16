use cosmic_text::{Align, Attrs, Family};
use tiny_skia::Pixmap;

use crate::{edges::margin::Margin, text::FontRenderer};

use super::interface::{
    component::{Component, ComponentContext, RenderParams},
    render_error,
    style::{ComponentStyle, RawComponentStyle},
};

pub struct Watermark {
    children: Vec<Box<dyn Component>>,
    value: Option<String>,
}

impl Component for Watermark {
    fn draw_self(
        &self,
        pixmap: &mut Pixmap,
        context: &ComponentContext,
        render_params: &RenderParams,
        _style: &ComponentStyle,
    ) -> render_error::Result<()> {
        let params = &context.take_snapshot_params;

        if let Some(value) = &params.watermark {
            let attrs = Attrs::new().family(Family::Name(
                &context.take_snapshot_params.watermark_font_family,
            ));

            FontRenderer::new(
                20.,
                20.,
                context.scale_factor,
                &context.take_snapshot_params.fonts_folder,
            )
            .draw_line(
                0.,
                render_params.y,
                pixmap.width() as f32,
                pixmap.height() as f32,
                &value,
                attrs,
                Some(Align::Center),
                pixmap,
            );
        }

        Ok(())
    }

    fn children(&self) -> &Vec<Box<dyn Component>> {
        &self.children
    }

    fn style(&self) -> RawComponentStyle {
        let default_style = RawComponentStyle::default();

        if self.value.is_some() {
            default_style.margin(Margin {
                bottom: 22.,
                top: 15.,
                ..Margin::default()
            })
        } else {
            default_style
        }
    }
}

impl Watermark {
    pub fn new(value: Option<String>) -> Watermark {
        Watermark {
            children: vec![],
            value,
        }
    }
}
