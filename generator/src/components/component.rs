use tiny_skia::Pixmap;

pub struct ComponentContext {
    pub scale_factor: f32,
}

pub struct ParentComponent {
    pub x: f32,
    pub y: f32,
}

pub trait Component {
    fn children(self, _component: Vec<Box<dyn Component>>) -> Self
    where
        Self: Sized,
    {
        self
    }

    fn get_children(&self) -> &Vec<Box<dyn Component>>;

    fn get_x(&self) -> f32 {
        0.
    }

    fn get_y(&self) -> f32 {
        0.
    }

    fn draw_self(
        &self,
        _parent_component: ParentComponent,
        _pixmap: &mut Pixmap,
        _context: &ComponentContext,
    ) {
    }

    fn draw(
        &self,
        parent_component: ParentComponent,
        pixmap: &mut Pixmap,
        context: &ComponentContext,
    ) {
        self.draw_self(parent_component, pixmap, context);

        for child in self.get_children() {
            child.draw(
                ParentComponent {
                    x: self.get_x(),
                    y: self.get_x(),
                },
                pixmap,
                context,
            )
        }
    }

    fn draw_root(&self, pixmap: &mut Pixmap, context: &ComponentContext) {
        self.draw(ParentComponent { x: 0., y: 0. }, pixmap, context)
    }
}
