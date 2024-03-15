use tiny_skia::{Color, GradientStop, LinearGradient, Paint, Point, Rect, SpreadMode, Transform};

use super::component::{Component, ComponentContext, ParentComponent};

pub struct Background {
    children: Vec<Box<dyn Component>>,
    gradient_stop_points: Vec<GradientStop>,
}

impl Background {
    pub fn create() -> Background {
        Background {
            children: vec![],
            gradient_stop_points: vec![
                GradientStop::new(0.0, Color::from_rgba8(58, 28, 113, 255)),
                GradientStop::new(0.5, Color::from_rgba8(215, 109, 119, 255)),
                GradientStop::new(1.0, Color::from_rgba8(255, 175, 123, 255)),
            ],
        }
    }
}

impl Component for Background {
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

    fn draw_self(
        &self,
        _: ParentComponent,
        pixmap: &mut tiny_skia::Pixmap,
        _context: &ComponentContext,
    ) {
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
    }
}
