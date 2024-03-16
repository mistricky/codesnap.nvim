use crate::{config::TakeSnapshotParams, snapshot::take_snapshot};
use nvim_oxi::{lua, Result};

pub fn save_snapshot(config: TakeSnapshotParams) -> Result<()> {
    match &config.save_path {
        Some(path) => {
            let pixmap = take_snapshot(config.clone())?;

            pixmap
                .save_png(path)
                .map_err(|err| nvim_oxi::Error::Lua(lua::Error::RuntimeError(err.to_string())))
        }
        None => Err(nvim_oxi::Error::Lua(lua::Error::RuntimeError(
            "Cannot find 'save_path' in config".to_string(),
        ))),
    }
}
