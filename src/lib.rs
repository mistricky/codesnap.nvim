use nvim_oxi as oxi;
use oxi::{print, Dictionary, Function, Result};

fn hello(arg: String) -> Result<String> {
    Ok(format!("Hello {arg}"))
}

fn say_hello(arg: String) -> Result<()> {
    print!("{:?}", hello(arg));
    Ok(())
}

#[oxi::module]
fn foo() -> oxi::Result<Dictionary> {
    Ok(Dictionary::from_iter([(
        "say_hello",
        Function::from_fn(say_hello),
    )]))
}
