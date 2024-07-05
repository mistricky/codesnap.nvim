use crate::{
    code::{calc_wh_with_min_width, prepare_code, CHAR_WIDTH},
    components::interface::{
        component::{Component, ComponentContext, RenderParams},
        render_error,
        style::{ComponentStyle, RawComponentStyle, Size, Style},
    },
    highlight::Highlight,
    text::FontRenderer,
};

pub struct Code {
    children: Vec<Box<dyn Component>>,
    line_height: f32,
    font_size: f32,
    value: String,
}

impl Component for Code {
    fn children(&self) -> &Vec<Box<dyn Component>> {
        &self.children
    }

    fn style(&self) -> RawComponentStyle {
        let (w, h) = calc_wh_with_min_width(&self.value, CHAR_WIDTH, self.line_height);

        Style::default().size(Size::Num(w), Size::Num(h))
    }

    fn draw_self(
        &self,
        pixmap: &mut tiny_skia::Pixmap,
        context: &ComponentContext,
        render_params: &RenderParams,
        style: &ComponentStyle,
        _parent_style: &ComponentStyle,
    ) -> render_error::Result<()> {
        let params = &context.take_snapshot_params;
        let highlight = Highlight::new(
            self.value.clone(),
            params.code_font_family.clone(),
            params.code_file_path.clone(),
            params.extension.clone(),
        );
        let highlight_result = highlight.parse(&params.themes_folder, &params.theme)?;

        FontRenderer::new(
            self.font_size,
            self.line_height,
            context.scale_factor,
            &context.take_snapshot_params.fonts_folder,
        )
        .draw_text(
            render_params.x,
            render_params.y,
            style.width,
            style.height,
            highlight_result.clone(),
            pixmap,
        );

        Ok(())
    }
}

impl Code {
    pub fn new(value: String, line_height: f32, font_size: f32) -> Code {
        Code {
            value: prepare_code(&value),
            line_height,
            font_size,
            children: vec![],
        }
    }
}
