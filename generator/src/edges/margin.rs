use super::edge::Edge;

#[derive(Clone, Default, Debug)]
pub struct Margin {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Edge for Margin {
    fn horizontal(&self) -> f32 {
        self.left + self.right
    }

    fn vertical(&self) -> f32 {
        self.bottom + self.top
    }
}

impl Margin {
    #[allow(dead_code)]
    pub fn from_value(value: f32) -> Margin {
        Margin {
            left: value,
            right: value,
            top: value,
            bottom: value,
        }
    }
}
