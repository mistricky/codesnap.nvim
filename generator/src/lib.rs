mod snapshot_config;

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
    fn snapshot_data(&self, image_snapshot: ImageSnapshot) -> LuaResult<SnapshotData> {
        let data = match self {
            SnapshotType::Png => image_snapshot.png_data(),
            SnapshotType::Svg => todo!(),
            SnapshotType::Html => todo!(),
        }
        .map_err(|_| mlua::Error::RuntimeError("Failed to generate snapshot data".to_string()))?;

        Ok(data)
    }
}

fn save(
    _: &Lua,
    (snapshot_type, path, config): (String, String, SnapshotConfigLua),
) -> LuaResult<()> {
    let image_snapshot = config
        .0
        .create_snapshot()
        .map_err(|_| mlua::Error::RuntimeError("Failed to create snapshot".to_string()))?;
    let snapshot_type: SnapshotType = snapshot_type.into();

    snapshot_type
        .snapshot_data(image_snapshot)?
        .save(&path)
        .map_err(|_| mlua::Error::RuntimeError(format!("Failed to save snapshot data to {}", path)))
}

fn copy_to_clipboard(_: &Lua, snapshot_type: SnapshotType) {}

#[mlua::lua_module(skip_memory_check)]
fn codesnap_generator(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    exports.set("save", lua.create_function(save)?)?;

    Ok(exports)
}
