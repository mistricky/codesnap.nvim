use crate::edges::{margin::Margin, padding::Padding};

#[derive(Clone, Debug)]
pub enum ComponentAlign {
    Row,
    Column,
}

pub enum Size {
    Dynamic,
    Num(f32),
}

#[derive(Clone, Debug)]
pub struct Style<T> {
    pub width: T,
    pub height: T,
    pub min_width: f32,
    pub align: ComponentAlign,
    pub padding: Padding,
    pub margin: Margin,
}

pub type RawComponentStyle = Style<Size>;
pub type ComponentStyle = Style<f32>;

impl Default for RawComponentStyle {
    fn default() -> Self {
        Style {
            min_width: 0.,
            width: Size::Dynamic,
            height: Size::Dynamic,
            align: ComponentAlign::Row,
            padding: Padding::default(),
            margin: Margin::default(),
        }
    }
}

impl Default for ComponentStyle {
    fn default() -> Self {
        Style {
            min_width: 0.,
            width: 0.,
            height: 0.,
            align: ComponentAlign::Row,
            padding: Padding::default(),
            margin: Margin::default(),
        }
    }
}

impl RawComponentStyle {
    pub fn size(mut self, width: Size, height: Size) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    // Only works if the width is calculate dynamically
    pub fn min_width(mut self, min_width: f32) -> Self {
        self.min_width = min_width;
        self
    }

    pub fn align(mut self, align: ComponentAlign) -> Self {
        self.align = align;
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn margin(mut self, margin: Margin) -> Self {
        self.margin = margin;
        self
    }
}
