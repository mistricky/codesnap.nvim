use super::component::{Component, ComponentContext};
use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Transform};

pub struct Rect {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    radius: f32,
    children: Vec<Box<dyn Component>>,
}

impl Component for Rect {
    fn get_children(&self) -> &Vec<Box<dyn Component>> {
        &self.children
    }

    fn children(mut self, components: Vec<Box<dyn Component>>) -> Self {
        self.children = components;
        self
    }

    fn draw_self(&self, pixmap: &mut Pixmap, context: &ComponentContext) {
        let mut path_builder = PathBuilder::new();
        let rect_width = self.w - 2. * self.radius;
        let rect_height = self.h - 2. * self.radius;

        path_builder.move_to(self.x + self.radius, self.y);
        path_builder.line_to(self.x + self.radius + rect_width, self.y);
        path_builder.line_to(self.x + self.radius + rect_width, self.y + self.radius);

        path_builder.line_to(self.x + rect_width + self.radius * 2., self.y + self.radius);

        path_builder.line_to(
            self.x + rect_width + self.radius * 2.,
            self.y + rect_height + self.radius,
        );
        path_builder.line_to(
            self.x + rect_width + self.radius,
            self.y + rect_height + self.radius,
        );
        path_builder.line_to(
            self.x + rect_width + self.radius,
            self.y + rect_height + self.radius * 2.,
        );

        path_builder.line_to(
            self.x + self.radius,
            self.y + rect_height + self.radius * 2.,
        );
        path_builder.line_to(self.x + self.radius, self.y + rect_height + self.radius);

        path_builder.line_to(self.x, self.y + rect_height + self.radius);

        path_builder.line_to(self.x, self.y + self.radius);
        path_builder.line_to(self.x + self.radius, self.y + self.radius);
        path_builder.line_to(self.x + self.radius, self.y);
        path_builder.line_to(self.x + self.radius + rect_width, self.y);
        path_builder.push_circle(
            self.x + rect_width + self.radius,
            self.y + rect_height + self.radius,
            self.radius,
        );
        path_builder.push_circle(
            self.x + self.radius + rect_width,
            self.y + self.radius,
            self.radius,
        );
        path_builder.push_circle(self.x + self.radius, self.y + self.radius, self.radius);
        path_builder.push_circle(
            self.x + self.radius,
            self.y + rect_height + self.radius,
            self.radius,
        );

        path_builder.close();
        let path = path_builder.finish().unwrap();
        let mut paint = Paint::default();
        paint.set_color_rgba8(40, 44, 52, 237);

        pixmap.fill_path(
            &path,
            &paint,
            FillRule::Winding,
            Transform::from_scale(context.scale_factor, context.scale_factor),
            None,
        );
    }
}

impl Rect {
    pub fn radius(mut self, radius_value: f32) -> Self {
        self.radius = radius_value;
        self
    }

    pub fn from_xywh(x: f32, y: f32, w: f32, h: f32) -> Self {
        Rect {
            x,
            y,
            w,
            h,
            radius: 0.,
            children: vec![],
        }
    }
}
