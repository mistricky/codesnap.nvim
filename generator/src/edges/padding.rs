use super::edge::Edge;

#[derive(Clone, Default, Debug)]
pub struct Padding {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Edge for Padding {
    fn horizontal(&self) -> f32 {
        self.left + self.right
    }

    fn vertical(&self) -> f32 {
        self.bottom + self.top
    }
}

impl Padding {
    pub fn from_value(value: f32) -> Padding {
        Padding {
            left: value,
            right: value,
            top: value,
            bottom: value,
        }
    }
}
