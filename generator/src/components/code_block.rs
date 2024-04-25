use crate::edges::margin::Margin;

use super::interface::{
    component::Component,
    style::{RawComponentStyle, Style},
};

pub struct CodeBlock {
    children: Vec<Box<dyn Component>>,
}

impl Component for CodeBlock {
    fn children(&self) -> &Vec<Box<dyn Component>> {
        &self.children
    }

    fn style(&self) -> RawComponentStyle {
        Style::default().margin(Margin {
            top: 10.,
            ..Margin::default()
        })
    }
}

impl CodeBlock {
    pub fn from_children(children: Vec<Box<dyn Component>>) -> CodeBlock {
        CodeBlock { children }
    }
}
