use tiny_skia::{
    Color, GradientStop, LinearGradient, Paint, Pixmap, Point, Rect, SpreadMode, Transform,
};

use super::interface::{
    component::{Component, ComponentContext, RenderParams},
    render_error,
    style::{ComponentAlign, ComponentStyle, RawComponentStyle},
};

pub struct Background {
    children: Vec<Box<dyn Component>>,
    gradient_stop_points: Vec<GradientStop>,
}

impl Background {
    pub fn from_children(children: Vec<Box<dyn Component>>) -> Background {
        Background {
            children,
            gradient_stop_points: vec![
                GradientStop::new(0.0, Color::from_rgba8(58, 28, 113, 255)),
                GradientStop::new(0.5, Color::from_rgba8(215, 109, 119, 255)),
                GradientStop::new(1.0, Color::from_rgba8(255, 175, 123, 255)),
            ],
        }
    }
}

impl Component for Background {
    fn children(&self) -> &Vec<Box<dyn Component>> {
        &self.children
    }

    fn style(&self) -> RawComponentStyle {
        RawComponentStyle::default().align(ComponentAlign::Column)
    }

    fn draw_self(
        &self,
        pixmap: &mut Pixmap,
        _context: &ComponentContext,
        _render_params: &RenderParams,
        _style: &ComponentStyle,
    ) -> render_error::Result<()> {
        let mut paint = Paint::default();
        let w = pixmap.width() as f32;
        let h = pixmap.height() as f32;

        paint.anti_alias = false;
        paint.shader = LinearGradient::new(
            Point::from_xy(0., 0.),
            Point::from_xy(w, 0.),
            self.gradient_stop_points.clone(),
            SpreadMode::Pad,
            Transform::identity(),
        )
        .unwrap();

        pixmap.fill_rect(
            Rect::from_xywh(0., 0., w, h).unwrap(),
            &paint,
            Transform::identity(),
            None,
        );

        Ok(())
    }
}
