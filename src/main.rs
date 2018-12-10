#[warn(missing_docs)]
extern crate quick_xml;
extern crate sdl2;
extern crate yaml_rust;
#[macro_use]
extern crate error_chain;

mod error;
mod item;
mod util;
mod window;

use std::path::Path;
use std::time::Duration;

use error::*;
use window::Window;

quick_main!(run);

fn run() -> Result<()> {
    // Create the items
    // TODO: Base off config
    let items: Vec<Box<item::Item>> = vec![Box::new(item::Command::new(
        vec!["echo".into(), "hello".into()],
        Duration::from_secs(5),
    )?)];

    // Start the window
    Window::with_callback(
        400,
        300,
        Path::new("/Library/Fonts/Comic Sans MS.ttf"),
        items,
    )
    .chain_err(|| "Failed to make window")?;
    Ok(())
}
