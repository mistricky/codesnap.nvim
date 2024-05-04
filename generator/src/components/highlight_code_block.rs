use super::{
    interface::{component::Component, style::ComponentStyle},
    rect::EDITOR_PADDING,
};
use tiny_skia::{Color, Paint, Rect, Transform};

#[derive(Default)]
pub struct HighlightCodeBlock {
    children: Vec<Box<dyn Component>>,
    line_height: f32,
    start_line_number: usize,
    end_line_number: usize,
    render_condition: bool,
}

impl Component for HighlightCodeBlock {
    fn children(&self) -> &Vec<Box<dyn Component>> {
        &self.children
    }

    fn render_condition(&self) -> bool {
        self.render_condition
    }

    fn draw_self(
        &self,
        pixmap: &mut tiny_skia::Pixmap,
        context: &super::interface::component::ComponentContext,
        render_params: &super::interface::component::RenderParams,
        _style: &super::interface::style::ComponentStyle,
        parent_style: &ComponentStyle,
    ) -> super::interface::render_error::Result<()> {
        let mut paint = Paint::default();
        let start_y_offset = (self.start_line_number - 1) as f32 * self.line_height;

        paint.anti_alias = false;
        paint.set_color(Color::from_rgba8(255, 255, 255, 10));
        pixmap.fill_rect(
            Rect::from_xywh(
                render_params.x - EDITOR_PADDING,
                render_params.y + start_y_offset,
                parent_style.width + EDITOR_PADDING * 2.,
                (self.end_line_number - self.start_line_number + 1) as f32 * self.line_height,
            )
            .unwrap(),
            &paint,
            Transform::from_scale(context.scale_factor, context.scale_factor),
            None,
        );

        Ok(())
    }
}

impl HighlightCodeBlock {
    pub fn from_line_number(
        start_line_number: Option<usize>,
        end_line_number: Option<usize>,
        line_height: f32,
    ) -> HighlightCodeBlock {
        if end_line_number < start_line_number {
            panic!("end_line_number should be greater than start_line_number")
        }

        match start_line_number {
            Some(start_line_number) => HighlightCodeBlock {
                render_condition: true,
                children: vec![],
                line_height,
                start_line_number,
                end_line_number: end_line_number.unwrap(),
            },
            None => HighlightCodeBlock::default(),
        }
    }
}
