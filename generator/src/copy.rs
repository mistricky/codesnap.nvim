use crate::{config::TakeSnapshotParams, snapshot::take_snapshot};
use arboard::{Clipboard, ImageData};

use nvim_oxi::Result;

pub fn copy_into_clipboard(config: TakeSnapshotParams) -> Result<()> {
    let pixmap = take_snapshot(config.clone())?;
    let premultplied_colors = pixmap.pixels();
    let colors = premultplied_colors
        .iter()
        .map(|premultplied_color| {
            vec![
                premultplied_color.red(),
                premultplied_color.green(),
                premultplied_color.blue(),
                premultplied_color.alpha(),
            ]
        })
        .flatten()
        .collect::<Vec<u8>>();
    let mut ctx = Clipboard::new().unwrap();

    let img_data = ImageData {
        width: pixmap.width() as usize,
        height: pixmap.height() as usize,
        bytes: colors.into(),
    };
    ctx.set_image(img_data).unwrap();

    Ok(())
}
