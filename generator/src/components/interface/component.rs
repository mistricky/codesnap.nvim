use super::{
    render_error,
    style::{ComponentAlign, ComponentStyle, RawComponentStyle, Size, Style},
};
use crate::{config::TakeSnapshotParams, edges::edge::Edge};
use std::sync::Arc;
use tiny_skia::Pixmap;

pub struct ComponentContext {
    pub scale_factor: f32,
    pub take_snapshot_params: Arc<TakeSnapshotParams>,
}

#[derive(Default, Clone)]
pub struct RenderParams {
    pub x: f32,
    pub y: f32,
}

#[derive(Default)]
pub struct ComponentRenderParams {
    pub parent_render_params: RenderParams,
    pub sibling_render_params: RenderParams,
}

impl ComponentRenderParams {
    fn parse_into_render_params_with_style(
        &self,
        parent_style: ComponentStyle,
        sibling_style: ComponentStyle,
        style: ComponentStyle,
    ) -> RenderParams {
        match parent_style.align {
            ComponentAlign::Row => RenderParams {
                x: self.sibling_render_params.x
                    + sibling_style.width
                    + style.margin.left
                    + sibling_style.padding.horizontal(),
                y: self.sibling_render_params.y + style.margin.top,
            },
            ComponentAlign::Column => RenderParams {
                x: self.sibling_render_params.x + style.margin.left,
                y: self.sibling_render_params.y
                    + style.margin.top
                    + sibling_style.height
                    + sibling_style.padding.vertical(),
            },
        }
    }
}

pub trait Component {
    fn children(&self) -> &Vec<Box<dyn Component>>;

    fn align(&self) -> ComponentAlign {
        ComponentAlign::Row
    }

    fn initialize(&self, render_params: &RenderParams) -> RenderParams {
        render_params.clone()
    }

    fn render_condition(&self) -> bool {
        true
    }

    fn draw_self(
        &self,
        _pixmap: &mut Pixmap,
        _context: &ComponentContext,
        _render_params: &RenderParams,
        _style: &ComponentStyle,
    ) -> render_error::Result<()> {
        Ok(())
    }

    fn style(&self) -> RawComponentStyle {
        RawComponentStyle::default()
    }

    fn parse_size(&self, size: Size, dynamic_value: f32) -> f32 {
        match size {
            Size::Num(num) => num,
            Size::Dynamic => dynamic_value,
        }
    }

    fn parsed_style(&self) -> Style<f32> {
        let style = self.style();
        let (width, height) = self.get_dynamic_wh();

        Style {
            width: self.parse_size(style.width, width)
                + style.padding.horizontal()
                + style.margin.horizontal(),
            height: self.parse_size(style.height, height)
                + style.padding.vertical()
                + style.margin.vertical(),
            align: style.align,
            padding: style.padding,
            margin: style.margin,
        }
    }

    fn draw(
        &self,
        pixmap: &mut Pixmap,
        context: &ComponentContext,
        component_render_params: ComponentRenderParams,
        parent_style: ComponentStyle,
        sibling_style: ComponentStyle,
    ) -> render_error::Result<RenderParams> {
        let style = self.parsed_style();
        let render_params = self.initialize(
            &component_render_params.parse_into_render_params_with_style(
                parent_style,
                sibling_style,
                style.clone(),
            ),
        );

        self.draw_self(pixmap, context, &render_params, &style)?;

        let children = self.children();
        let mut sibling_render_params = RenderParams {
            x: render_params.x + style.padding.left,
            y: render_params.y + style.padding.top,
        };
        let mut sibling_style = ComponentStyle::default();

        for child in children {
            if !child.render_condition() {
                continue;
            }

            sibling_render_params = child.draw(
                pixmap,
                context,
                ComponentRenderParams {
                    parent_render_params: render_params.clone(),
                    sibling_render_params,
                },
                style.clone(),
                sibling_style,
            )?;
            sibling_style = child.parsed_style();
        }

        Ok(render_params.clone())
    }

    // Dynamic calculate width and height of children, if the children is empty, get_dynamic_wh
    // will return (0., 0.)
    fn get_dynamic_wh(&self) -> (f32, f32) {
        let children = self.children();
        let calc_children_wh = |cb: fn((f32, f32), &Box<dyn Component>) -> (f32, f32)| {
            children.iter().fold((0., 0.), cb)
        };
        let style = self.style();

        match style.align {
            // If align is row, width is sum of children width, height is max of children height
            ComponentAlign::Row => calc_children_wh(|(w, h), child| {
                let style = child.parsed_style();

                (w + style.width, h.max(style.height))
            }),
            // If align is column, width is max of children width, height is sum of children height
            ComponentAlign::Column => calc_children_wh(|(w, h), child| {
                let style = child.parsed_style();

                (w.max(style.width), h + style.height)
            }),
        }
    }
}
