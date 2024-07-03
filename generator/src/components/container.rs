use tiny_skia::Pixmap;

use super::interface::{
    component::{Component, ComponentContext, ComponentRenderParams},
    render_error::Result,
    style::Style,
};

pub struct Container {
    children: Vec<Box<dyn Component>>,
}

impl Component for Container {
    fn children(&self) -> &Vec<Box<dyn Component>> {
        &self.children
    }
}

impl Container {
    pub fn from_children(children: Vec<Box<dyn Component>>) -> Container {
        Container { children }
    }

    pub fn draw_root(&self, context: &ComponentContext) -> Result<Pixmap> {
        let style = self.parsed_style();
        let mut pixmap = Pixmap::new(
            (style.width * context.scale_factor) as u32,
            (style.height * context.scale_factor) as u32,
        )
        .unwrap();

        self.draw(
            &mut pixmap,
            context,
            ComponentRenderParams::default(),
            Style::default(),
            Style::default(),
        )?;

        Ok(pixmap)
    }
}
