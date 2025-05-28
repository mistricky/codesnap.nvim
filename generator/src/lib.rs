mod snapshot_config;

use std::{ffi::OsStr, path::Path};

use codesnap::snapshot::{image_snapshot::ImageSnapshot, snapshot_data::SnapshotData};
use mlua::prelude::*;
use snapshot_config::SnapshotConfigLua;

enum SnapshotType {
    Png,
    Svg,
    Html,
}

impl From<String> for SnapshotType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "png" => SnapshotType::Png,
            "svg" => SnapshotType::Svg,
            "html" => SnapshotType::Html,
            _ => SnapshotType::Png,
        }
    }
}

impl SnapshotType {
    fn snapshot_data(
        &self,
        image_snapshot: ImageSnapshot,
        is_raw: bool,
    ) -> LuaResult<SnapshotData> {
        let data = match self {
            SnapshotType::Png => {
                if is_raw {
                    image_snapshot.raw_data()
                } else {
                    image_snapshot.png_data()
                }
            }
            SnapshotType::Svg => image_snapshot.svg_data(),
            SnapshotType::Html => image_snapshot.html_data(),
        }
        .map_err(|_| mlua::Error::RuntimeError("Failed to generate snapshot data".to_string()))?;

        Ok(data)
    }
}

fn create_image_snapshot_by_config(config: &SnapshotConfigLua) -> LuaResult<ImageSnapshot> {
    config
        .0
        .create_snapshot()
        .map_err(|_| mlua::Error::RuntimeError("Failed to create image snapshot".to_string()))
}

fn save(_: &Lua, (file_path, config): (String, SnapshotConfigLua)) -> LuaResult<()> {
    let snapshot_type: SnapshotType = Path::new(&file_path)
        .extension()
        .and_then(OsStr::to_str)
        .ok_or_else(|| mlua::Error::RuntimeError("Invalid file extension".to_string()))?
        .to_string()
        .into();

    SnapshotType::from(snapshot_type)
        .snapshot_data(create_image_snapshot_by_config(&config)?, false)?
        .save(&file_path)
        .map_err(|_| {
            mlua::Error::RuntimeError(format!("Failed to save snapshot data to {}", file_path))
        })
}

fn copy(_: &Lua, config: SnapshotConfigLua) -> LuaResult<()> {
    create_image_snapshot_by_config(&config)?
        .raw_data()
        .map_err(|_| {
            mlua::Error::RuntimeError(format!("Failed to generate snapshot data for clipboard"))
        })?
        .copy()
        .map_err(|_| {
            mlua::Error::RuntimeError("Failed to copy snapshot data to clipboard".to_string())
        })
}

fn copy_ascii(_: &Lua, config: SnapshotConfigLua) -> LuaResult<()> {
    config
        .0
        .create_ascii_snapshot()
        .map_err(|_| mlua::Error::RuntimeError("Failed to create ASCII snapshot".to_string()))?
        .raw_data()
        .map_err(|_| {
            mlua::Error::RuntimeError("Failed to generate ASCII snapshot data".to_string())
        })?
        .copy()
        .map_err(|_| {
            mlua::Error::RuntimeError("Failed to copy ASCII snapshot to clipboard".to_string())
        })?;

    Ok(())
}

#[mlua::lua_module(skip_memory_check)]
fn generator(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    exports.set("save", lua.create_function(save)?)?;
    exports.set("copy", lua.create_function(copy)?)?;
    exports.set("copy_ascii", lua.create_function(copy_ascii)?)?;

    Ok(exports)
}
