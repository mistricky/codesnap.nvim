use tiny_skia::{Color, FillRule, Paint, PathBuilder, Transform};

use crate::components::{
    component::{Component, ComponentContext, ParentComponent},
    render_error::{self},
};

pub struct MacTitleBar {
    x: f32,
    y: f32,
    r: f32,
    children: Vec<Box<dyn Component>>,
}

impl Component for MacTitleBar {
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
        context: &ComponentContext,
    ) -> render_error::Result<()> {
        self.draw_control_buttons(
            pixmap,
            vec![
                Color::from_rgba8(255, 94, 87, 255),
                Color::from_rgba8(255, 186, 46, 255),
                Color::from_rgba8(43, 200, 65, 255),
            ],
            25.,
            Transform::from_scale(context.scale_factor, context.scale_factor),
        );

        Ok(())
    }
}

impl MacTitleBar {
    pub fn from_xyr(x: f32, y: f32, r: f32) -> MacTitleBar {
        MacTitleBar {
            x,
            y,
            r,
            children: vec![],
        }
    }

    fn draw_control_buttons(
        &self,
        pixmap: &mut tiny_skia::Pixmap,
        colors: Vec<Color>,
        gap: f32,
        transform: Transform,
    ) {
        for (index, color) in colors.into_iter().enumerate() {
            self.draw_control_button(pixmap, color, index as f32 * gap, transform);
        }
    }

    fn draw_control_button(
        &self,
        pixmap: &mut tiny_skia::Pixmap,
        color: Color,
        x_offset: f32,
        transform: Transform,
    ) {
        let mut path_builder = PathBuilder::new();

        path_builder.push_circle(self.x + x_offset, self.y, self.r);
        path_builder.close();
        let path = path_builder.finish().unwrap();
        let mut paint = Paint::default();
        paint.set_color(color);

        pixmap.fill_path(&path, &paint, FillRule::Winding, transform, None);
    }
}
