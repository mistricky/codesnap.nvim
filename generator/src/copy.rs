use crate::{config::TakeSnapshotParams, snapshot::take_snapshot};
#[cfg(target_os = "linux")]
use arboard::SetExtLinux;
use arboard::{Clipboard, ImageData};

use nvim_oxi::Result;

// The function will be called as FFI on Lua side
#[allow(dead_code)]
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

    let img_data = ImageData {
        width: pixmap.width() as usize,
        height: pixmap.height() as usize,
        bytes: colors.into(),
    };

    #[cfg(target_os = "linux")]
    std::thread::spawn(move || {
        Clipboard::new()
            .unwrap()
            .set()
            .wait()
            .image(img_data)
            .unwrap();
    });

    #[cfg(not(target_os = "linux"))]
    Clipboard::new().unwrap().set_image(img_data).unwrap();

    Ok(())
}
