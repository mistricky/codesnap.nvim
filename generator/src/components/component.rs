use tiny_skia::{Pixmap, Transform};

pub struct ComponentContext {
    pub scale_factor: f32,
}

pub trait Component {
    fn children(self, component: Vec<Box<dyn Component>>) -> Self
    where
        Self: Sized;

    fn get_children(&self) -> &Vec<Box<dyn Component>>;

    fn draw_self(&self, _pixmap: &mut Pixmap, _context: &ComponentContext) {}

    fn draw(&self, pixmap: &mut Pixmap, context: &ComponentContext) {
        self.draw_self(pixmap, context);

        for child in self.get_children() {
            child.draw(pixmap, context)
        }
    }
}
