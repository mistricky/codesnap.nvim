pub mod code;
mod mac_title_bar;

use crate::{code::calc_wh, padding::Padding};

use self::{code::Code, mac_title_bar::MacTitleBar};
use super::{component::Component, rect::Rect, text::Text};

const CONTROL_BAR_RADIUS: f32 = 8.;

pub struct Editor {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    children: Vec<Box<dyn Component>>,
    has_column_number: bool,
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
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Editor {
        Editor {
            x,
            y,
            w,
            h,
            padding: Padding {
                left: 0.,
                right: 0.,
                top: 0.,
                bottom: 0.,
            },
            children: vec![],
            has_column_number: false,
            code_y_offset: 0.,
        }
    }

    pub fn render_mac_title_bar(mut self) -> Self {
        let mac_title_bar = MacTitleBar::from_xyr(
            self.x + CONTROL_BAR_RADIUS + self.padding.left,
            self.y + CONTROL_BAR_RADIUS + self.padding.top,
            CONTROL_BAR_RADIUS,
        );

        self.children.push(Box::new(mac_title_bar));
        self
    }

    pub fn column_number(mut self) -> Self {
        self.has_column_number = true;
        self
    }

    pub fn title(mut self, title: Option<String>) -> Self {
        // if let Some(title) = title {
        //     let (width, height) = calc_wh(&title, 9.05, 20.);
        //     let x = (self.w + self.padding.horizontal() - width) / 2. + self.x;
        //     let title = Text::new(
        //         x,
        //         self.y,
        //         width,
        //         height,
        //         title,
        //         "Hack Nerd Font".to_string(),
        //         12.,
        //         15.,
        //     );
        //
        //     self.children.push(Box::new(title));
        // }

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

    pub fn header_height(&self) -> f32 {
        CONTROL_BAR_RADIUS * 2.
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
        self.code_y_offset = code_y_offset;
        self
    }
}