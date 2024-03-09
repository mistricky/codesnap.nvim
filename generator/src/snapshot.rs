use tiny_skia::Pixmap;

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

const SCALE_FACTOR: f32 = 3.;

pub fn take_snapshot(params: TakeSnapshotParams) -> Pixmap {
    let (width, height) = calc_wh(&params.code, 9.05, 20.);
    let pixmap_vertical_padding = 82.;
    let pixmap_horizontal_padding = 122.;
    let padding = Padding {
        left: 20.,
        right: 20.,
        top: 20.,
        bottom: 28.,
    };

    let editor = Editor::new(
        pixmap_horizontal_padding,
        pixmap_vertical_padding,
        width,
        height,
    )
    .code_y_offset(10.)
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
    )
    .title(params.title);

    let pixmap_width = (pixmap_horizontal_padding * 2. + width + padding.horizontal()) as u32;
    let pixmap_height =
        (pixmap_vertical_padding * 2. + height + padding.vertical() + editor.header_height())
            as u32;

    let mut pixmap = Pixmap::new(
        pixmap_width * SCALE_FACTOR as u32,
        pixmap_height * SCALE_FACTOR as u32,
    )
    .unwrap();
    let context = ComponentContext {
        scale_factor: SCALE_FACTOR,
    };
    let background = Background::create().children(vec![Box::new(editor)]);

    background.draw(&mut pixmap, &context);

    return pixmap;
}
