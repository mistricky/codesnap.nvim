use tiny_skia::{
    Color, GradientStop, LinearGradient, Paint, Pixmap, Point, Rect, SpreadMode, Transform,
};

use super::interface::{
    component::{Component, ComponentContext, RenderParams},
    render_error::{self, RenderError},
    style::{ComponentAlign, ComponentStyle, RawComponentStyle},
};

pub struct Background {
    children: Vec<Box<dyn Component>>,
}

impl Background {
    pub fn from_children(children: Vec<Box<dyn Component>>) -> Background {
        Background { children }
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
        context: &ComponentContext,
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
            Background::get_theme(&context.take_snapshot_params.bg_theme)?,
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

impl Background {
    fn get_theme(theme: &str) -> render_error::Result<Vec<GradientStop>> {
        let theme = match theme {
            "default" => vec![
                GradientStop::new(0.0, Color::from_rgba8(58, 28, 113, 255)),
                GradientStop::new(0.5, Color::from_rgba8(215, 109, 119, 255)),
                GradientStop::new(0.95, Color::from_rgba8(255, 175, 123, 255)),
            ],
            "cyan" => vec![
                GradientStop::new(0.0, Color::from_rgba8(31, 162, 255, 255)),
                GradientStop::new(0.4, Color::from_rgba8(18, 216, 250, 255)),
                GradientStop::new(0.95, Color::from_rgba8(166, 255, 203, 255)),
            ],
            "rose" => vec![
                GradientStop::new(0.1, Color::from_rgba8(180, 101, 218, 255)),
                GradientStop::new(0.33, Color::from_rgba8(207, 108, 201, 255)),
                GradientStop::new(0.66, Color::from_rgba8(238, 96, 156, 255)),
                GradientStop::new(1., Color::from_rgba8(238, 96, 156, 255)),
            ],
            "pink" => vec![
                GradientStop::new(0.22, Color::from_rgba8(221, 94, 137, 255)),
                GradientStop::new(0.55, Color::from_rgba8(247, 187, 151, 255)),
            ],
            "yellow" => vec![
                GradientStop::new(0., Color::from_rgba8(85, 239, 196, 255)),
                GradientStop::new(0.4, Color::from_rgba8(255, 234, 167, 255)),
            ],
            "blue" => vec![
                GradientStop::new(0.28, Color::from_rgba8(248, 165, 194, 255)),
                GradientStop::new(0.95, Color::from_rgba8(116, 185, 255, 255)),
            ],
            _ => return Err(RenderError::UnknownBackgroundTheme(theme.to_string())),
        };

        Ok(theme)
    }
}
