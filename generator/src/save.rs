use crate::{config::TakeSnapshotParams, snapshot::take_snapshot};
use nvim_oxi::{print, Result};

pub fn save_snapshot(config: TakeSnapshotParams) -> Result<()> {
    match &config.save_path {
        Some(path) => {
            take_snapshot(config.clone())
                .save_png(path)
                .unwrap_or_else(|path| {
                    print!("Save {} failed!", path);
                });
        }
        None => {
            print!("Cannot find 'save_path' in config");
        }
    };

    Ok(())
}
