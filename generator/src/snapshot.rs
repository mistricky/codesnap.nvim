use std::sync::Arc;

use tiny_skia::Pixmap;

use crate::components::background::Background;
use crate::components::breadcrumbs::Breadcrumbs;
use crate::components::code_block::CodeBlock;
use crate::components::container::Container;
use crate::components::editor::code::Code;
use crate::components::editor::mac_title_bar::MacTitleBar;
use crate::components::highlight_code_block::HighlightCodeBlock;
use crate::components::interface::component::ComponentContext;
use crate::components::interface::render_error;
use crate::components::line_number::LineNumber;
use crate::components::rect::Rect;
use crate::components::watermark::Watermark;
use crate::config::TakeSnapshotParams;

// Scale the screenshot to 3 times its size
const SCALE_FACTOR: f32 = 3.;
const LINE_HEIGHT: f32 = 20.;
const VIEW_WATERMARK_PADDING: f32 = 82.;

// The params is come from neovim instance
pub fn take_snapshot(params: TakeSnapshotParams) -> render_error::Result<Pixmap> {
    let context = ComponentContext {
        scale_factor: SCALE_FACTOR,
        take_snapshot_params: Arc::new(params.clone()),
    };
    let background_padding = Background::parse_background_padding(
        params.bg_x_padding,
        params.bg_y_padding,
        params.bg_padding,
    );

    // If vertical background padding is less than 82., should hidden watermark component
    // If watermark text is equal to "", the watermark component is hidden
    let watermark = if background_padding.bottom >= VIEW_WATERMARK_PADDING {
        params.watermark
    } else {
        "".to_string()
    };
    let pixmap = Container::from_children(vec![Box::new(Background::new(
        background_padding,
        vec![
            Box::new(Rect::new(
                16.,
                params.min_width,
                vec![
                    Box::new(MacTitleBar::from_radius(8., params.mac_window_bar)),
                    Box::new(Breadcrumbs::from_path(
                        params.file_path,
                        15.,
                        params.breadcrumbs_separator,
                        params.has_breadcrumbs,
                    )),
                    Box::new(CodeBlock::from_children(vec![
                        Box::new(HighlightCodeBlock::from_line_number(
                            params.highlight_start_line_number,
                            params.highlight_end_line_number,
                            LINE_HEIGHT,
                        )),
                        Box::new(LineNumber::new(
                            &params.code,
                            params.start_line_number,
                            LINE_HEIGHT,
                        )),
                        Box::new(Code::new(params.code, LINE_HEIGHT, 15.)),
                    ])),
                ],
            )),
            Box::new(Watermark::new(watermark)),
        ],
    ))])
    .draw_root(&context)?;

    Ok(pixmap)
}
