use crate::{config::TakeSnapshotParams, snapshot::take_snapshot};
use nvim_oxi::{print, Result};

pub fn save_snapshot(config: TakeSnapshotParams) -> Result<()> {
    match &config.save_path {
        Some(path) => {
            let _ = take_snapshot(config.clone()).save_png(path);
        }
        None => {
            print!("Cannot find 'save_path' in config");
        }
    };

    Ok(())
}
