pub mod code;
mod mac_title_bar;

use crate::padding::Padding;

use self::{code::Code, mac_title_bar::MacTitleBar};
use super::{component::Component, rect::Rect};

const CONTROL_BAR_RADIUS: f32 = 8.;

pub struct Editor {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    children: Vec<Box<dyn Component>>,
    has_title_bar: bool,
    padding: Padding,
    code_y_offset: f32,
}

impl Component for Editor {
    fn children(mut self, components: Vec<Box<dyn Component>>) -> Self
    where
        Self: Sized,
    {
        self.children.extend(components);
        self
    }

    fn get_children(&self) -> &Vec<Box<dyn Component>> {
        &self.children
    }
}

impl Editor {
    pub fn new(x: f32, y: f32, w: f32, h: f32, has_title_bar: bool) -> Editor {
        Editor {
            x,
            y,
            w,
            h,
            has_title_bar,
            padding: Padding {
                left: 0.,
                right: 0.,
                top: 0.,
                bottom: 0.,
            },
            children: vec![],
            code_y_offset: 0.,
        }
    }

    pub fn render_mac_title_bar(mut self) -> Self {
        if self.has_title_bar {
            let mac_title_bar = MacTitleBar::from_xyr(
                self.x + CONTROL_BAR_RADIUS + self.padding.left,
                self.y + CONTROL_BAR_RADIUS + self.padding.top,
                CONTROL_BAR_RADIUS,
            );

            self.children.push(Box::new(mac_title_bar));
        }

        self
    }

    pub fn render_editor(mut self, radius: f32) -> Self {
        let roundrect = Rect::from_xywh(
            self.x,
            self.y,
            self.w + self.padding.horizontal(),
            self.h + self.header_height() + self.padding.vertical() + self.code_y_offset,
        )
        .radius(radius);
        self.children.push(Box::new(roundrect));

        self
    }

    pub fn width(&self) -> f32 {
        self.w + self.padding.horizontal()
    }

    pub fn height(&self) -> f32 {
        self.h + self.header_height() + self.padding.vertical() + self.code_y_offset
    }

    pub fn header_height(&self) -> f32 {
        if self.has_title_bar {
            CONTROL_BAR_RADIUS * 2.
        } else {
            0.
        }
    }

    pub fn render_code(
        mut self,
        code: &str,
        language: Option<String>,
        extension: Option<String>,
        line_height: f32,
        font_size: f32,
        font_family: &str,
    ) -> Self {
        let code = Code::new(
            self.x + self.padding.left,
            self.y + self.header_height() + self.padding.top + self.code_y_offset,
            self.w,
            self.h,
            code.to_string(),
            language,
            extension,
            font_family.to_string(),
        )
        .line_height(line_height)
        .font_size(font_size);

        self.children.push(Box::new(code));
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn code_y_offset(mut self, code_y_offset: f32) -> Self {
        if self.has_title_bar {
            self.code_y_offset = code_y_offset;
        }

        self
    }
}
