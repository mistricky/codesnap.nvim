use codesnap::config::SnapshotConfig;
use mlua::{FromLua, LuaSerdeExt};

pub struct SnapshotConfigLua(pub SnapshotConfig);

impl FromLua for SnapshotConfigLua {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        let config: SnapshotConfig = lua.from_value::<SnapshotConfig>(value)?;

        Ok(SnapshotConfigLua(config))
    }
}
