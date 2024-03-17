use nvim_oxi::conversion::{Error as ConversionError, FromObject, ToObject};
use nvim_oxi::serde::{Deserializer, Serializer};
use nvim_oxi::{lua, Object};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct TakeSnapshotParams {
    pub mac_window_bar: bool,
    pub watermark: String,
    pub title: Option<String>,
    pub code_font_family: String,
    pub watermark_font_family: String,
    pub code: String,
    pub language: Option<String>,
    pub extension: Option<String>,
    pub save_path: Option<String>,
    pub themes_folder: String,
    pub fonts_folder: String,
    pub theme: String,
    pub bg_theme: String,
}

impl FromObject for TakeSnapshotParams {
    fn from_object(obj: Object) -> Result<Self, ConversionError> {
        Self::deserialize(Deserializer::new(obj)).map_err(Into::into)
    }
}

impl ToObject for TakeSnapshotParams {
    fn to_object(self) -> Result<Object, ConversionError> {
        self.serialize(Serializer::new()).map_err(Into::into)
    }
}

impl lua::Poppable for TakeSnapshotParams {
    unsafe fn pop(lstate: *mut lua::ffi::lua_State) -> Result<Self, lua::Error> {
        let obj = Object::pop(lstate)?;
        Self::from_object(obj).map_err(lua::Error::pop_error_from_err::<Self, _>)
    }
}

impl lua::Pushable for TakeSnapshotParams {
    unsafe fn push(self, lstate: *mut lua::ffi::lua_State) -> Result<std::ffi::c_int, lua::Error> {
        self.to_object()
            .map_err(lua::Error::push_error_from_err::<Self, _>)?
            .push(lstate)
    }
}
