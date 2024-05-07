use super::interface::{
    component::{Component, ComponentContext, RenderParams},
    render_error,
    style::{ComponentAlign, ComponentStyle, RawComponentStyle, Style},
};
use crate::edges::padding::Padding;
use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Transform};

pub const EDITOR_PADDING: f32 = 20.;

pub struct Rect {
    radius: f32,
    min_width: f32,
    children: Vec<Box<dyn Component>>,
}

impl Component for Rect {
    fn children(&self) -> &Vec<Box<dyn Component>> {
        &self.children
    }

    fn style(&self) -> RawComponentStyle {
        Style::default()
            .min_width(self.min_width)
            .align(ComponentAlign::Column)
            .padding(Padding::from_value(EDITOR_PADDING))
    }

    fn draw_self(
        &self,
        pixmap: &mut Pixmap,
        context: &ComponentContext,
        render_params: &RenderParams,
        style: &ComponentStyle,
        _parent_style: &ComponentStyle,
    ) -> render_error::Result<()> {
        let mut path_builder = PathBuilder::new();
        let x = render_params.x;
        let y = render_params.y;
        let w = style.width;
        let h = style.height;

        let rect_width = w - 2. * self.radius;
        let rect_height = h - 2. * self.radius;

        path_builder.move_to(x + self.radius, y);
        path_builder.line_to(x + self.radius + rect_width, y);
        path_builder.line_to(x + self.radius + rect_width, y + self.radius);

        path_builder.line_to(x + rect_width + self.radius * 2., y + self.radius);

        path_builder.line_to(
            x + rect_width + self.radius * 2.,
            y + rect_height + self.radius,
        );
        path_builder.line_to(x + rect_width + self.radius, y + rect_height + self.radius);
        path_builder.line_to(
            x + rect_width + self.radius,
            y + rect_height + self.radius * 2.,
        );

        path_builder.line_to(x + self.radius, y + rect_height + self.radius * 2.);
        path_builder.line_to(x + self.radius, y + rect_height + self.radius);

        path_builder.line_to(x, y + rect_height + self.radius);

        path_builder.line_to(x, y + self.radius);
        path_builder.line_to(x + self.radius, y + self.radius);
        path_builder.line_to(x + self.radius, y);
        path_builder.line_to(x + self.radius + rect_width, y);
        path_builder.push_circle(
            x + rect_width + self.radius,
            y + rect_height + self.radius,
            self.radius,
        );
        path_builder.push_circle(x + self.radius + rect_width, y + self.radius, self.radius);
        path_builder.push_circle(x + self.radius, y + self.radius, self.radius);
        path_builder.push_circle(x + self.radius, y + rect_height + self.radius, self.radius);

        path_builder.close();
        let path = path_builder.finish().unwrap();
        let mut paint = Paint::default();
        paint.set_color_rgba8(40, 44, 52, 237);

        pixmap.fill_path(
            &path,
            &paint,
            FillRule::Winding,
            Transform::from_scale(context.scale_factor, context.scale_factor),
            // Transform::identity(),
            None,
        );

        Ok(())
    }
}

impl Rect {
    pub fn new(radius: f32, min_width: Option<f32>, children: Vec<Box<dyn Component>>) -> Rect {
        Rect {
            radius,
            children,
            min_width: min_width.unwrap_or(0.),
        }
    }
}
