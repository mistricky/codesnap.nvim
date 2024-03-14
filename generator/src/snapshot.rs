use tiny_skia::Pixmap;

use crate::components::watermark::Watermark;
use crate::config::TakeSnapshotParams;
use crate::{
    code::calc_wh,
    components::{
        background::Background,
        component::{Component, ComponentContext},
        editor::Editor,
    },
    padding::Padding,
};

// Scale the screenshot to 3 times its size
const SCALE_FACTOR: f32 = 3.;

// The params is come from neovim instance
pub fn take_snapshot(params: TakeSnapshotParams) -> Pixmap {
    let (width, height) = calc_wh(&params.code, 9.05, 20.);
    let pixmap_vertical_padding = 82.;
    let pixmap_horizontal_padding = 122.;
    // Padding of editor shape
    let padding = Padding {
        left: 20.,
        right: 20.,
        top: 20.,
        bottom: 24.,
    };

    let editor = Editor::new(
        pixmap_horizontal_padding,
        pixmap_vertical_padding,
        width,
        height,
        params.mac_window_bar,
    )
    .code_y_offset(15.)
    .padding(padding.clone())
    .render_editor(16.)
    .render_mac_title_bar()
    .render_code(
        &params.code,
        params.language,
        params.extension,
        20.,
        15.,
        &params.code_font_family,
    );

    let pixmap_width = (pixmap_horizontal_padding * 2. + editor.width()) as u32;
    let pixmap_height = (pixmap_vertical_padding * 2. + editor.height()) as u32;
    let (watermark_bottom_margin, watermark_offset_y) = params
        .watermark
        .as_ref()
        .map(|_| (200., 40.))
        .unwrap_or((0., 0.));

    let mut pixmap = Pixmap::new(
        pixmap_width * SCALE_FACTOR as u32,
        (pixmap_height as f32 * SCALE_FACTOR + watermark_bottom_margin - watermark_offset_y) as u32,
    )
    .unwrap();
    let context = ComponentContext {
        scale_factor: SCALE_FACTOR,
    };
    let watermark = Watermark::new(
        params.watermark,
        params.watermark_font_family,
        watermark_bottom_margin,
    );
    let background = Background::create().children(vec![Box::new(editor), Box::new(watermark)]);

    background.draw(&mut pixmap, &context);

    return pixmap;
}
