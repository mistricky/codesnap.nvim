use super::interface::component::Component;

pub struct CodeBlock {
    children: Vec<Box<dyn Component>>,
}

impl Component for CodeBlock {
    fn children(&self) -> &Vec<Box<dyn Component>> {
        &self.children
    }
}

impl CodeBlock {
    pub fn from_children(children: Vec<Box<dyn Component>>) -> CodeBlock {
        CodeBlock { children }
    }
}
