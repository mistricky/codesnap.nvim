use cosmic_text::{Attrs, Color, Family};
use regex::Regex;

use crate::{code::calc_wh_with_min_width, edges::margin::Margin, text::FontRenderer};

use super::interface::{
    component::Component,
    style::{ComponentStyle, RawComponentStyle, Size},
};

pub struct Breadcrumbs {
    children: Vec<Box<dyn Component>>,
    path: String,
    line_height: f32,
    has_breadcrumbs: bool,
}

impl Component for Breadcrumbs {
    fn children(&self) -> &Vec<Box<dyn Component>> {
        &self.children
    }

    fn style(&self) -> RawComponentStyle {
        let style = RawComponentStyle::default();

        if self.has_breadcrumbs {
            let (w, h) = calc_wh_with_min_width(&self.path, 8., self.line_height);

            style.size(Size::Num(w), Size::Num(h)).margin(Margin {
                top: 5.,
                ..Margin::default()
            })
        } else {
            style
        }
    }

    fn draw_self(
        &self,
        pixmap: &mut tiny_skia::Pixmap,
        context: &super::interface::component::ComponentContext,
        render_params: &super::interface::component::RenderParams,
        style: &super::interface::style::ComponentStyle,
        _parent_style: &ComponentStyle,
    ) -> super::interface::render_error::Result<()> {
        if self.has_breadcrumbs {
            let attrs = Attrs::new()
                .color(Color::rgb(128, 132, 139))
                .family(Family::Name(&context.take_snapshot_params.code_font_family));

            FontRenderer::new(
                12.,
                self.line_height,
                context.scale_factor,
                &context.take_snapshot_params.fonts_folder,
            )
            .draw_text(
                render_params.x,
                render_params.y,
                style.width,
                self.line_height,
                vec![(&self.path, attrs)],
                pixmap,
            );
        }

        Ok(())
    }
}

impl Breadcrumbs {
    pub fn from_path(
        path: String,
        line_height: f32,
        separator: String,
        has_breadcrumbs: bool,
    ) -> Breadcrumbs {
        let path = Regex::new("/")
            .unwrap()
            .replace_all(&path, separator)
            .to_string();

        Breadcrumbs {
            children: vec![],
            path,
            line_height,
            has_breadcrumbs,
        }
    }
}
