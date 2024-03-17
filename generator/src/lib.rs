mod code;
mod color;
mod components;
mod config;
mod copy;
mod edges;
mod highlight;
mod path;
mod save;
mod snapshot;
mod text;

use copy::copy_into_clipboard;
use nvim_oxi::{Dictionary, Function, Result};
use save::save_snapshot;

#[nvim_oxi::module]
fn generator() -> Result<Dictionary> {
    Ok(Dictionary::from_iter([
        (
            "copy_into_clipboard",
            Function::from_fn(copy_into_clipboard),
        ),
        ("save_snapshot", Function::from_fn(save_snapshot)),
    ]))
}
