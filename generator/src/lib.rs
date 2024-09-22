mod code;
mod color;
mod components;
mod config;
mod copy;
mod copy_ascii;
mod edges;
mod highlight;
mod path;
mod save;
mod snapshot;
mod text;

use config::TakeSnapshotParams;
use copy::copy_into_clipboard;
use copy_ascii::copy_ascii;
use nvim_oxi::{api, Dictionary, Function};
use save::save_snapshot;

#[nvim_oxi::plugin]
fn generator() -> nvim_oxi::Result<Dictionary> {
    let copy_into_clipboard: Function<TakeSnapshotParams, Result<(), api::Error>> =
        Function::from_fn(copy_into_clipboard);

    Ok(Dictionary::from_iter([
        ("copy_into_clipboard", copy_into_clipboard),
        ("save_snapshot", Function::from_fn(save_snapshot)),
        ("copy_ascii", Function::from_fn(copy_ascii)),
    ]))
}
