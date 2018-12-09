//! Floating window "panel". See more
//! [here](https://github.com/mishajw/incredi-panel).

#![warn(missing_docs)]

extern crate quick_xml;
extern crate yaml_rust;
#[macro_use]
extern crate error_chain;
extern crate sfml;
#[macro_use]
extern crate log;

mod error;
mod item;
mod util;
mod window;

use std::time::Duration;

use error::*;
use window::Window;

quick_main!(run);

fn run() -> Result<()> {
    // Initialize logging
    let env = env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, "trace");
    env_logger::Builder::from_env(env).init();

    // Create the items
    // TODO: Base off config
    let items: Vec<Box<item::Item>> = vec![Box::new(item::Command::new(
        vec!["echo".into(), "-n".into(), "hello".into()],
        Duration::from_secs(5),
    ))];

    // Start the window
    Window::start(
        400,
        300,
        "/usr/share/fonts/TTF/Yrsa-Regular.ttf",
        items,
    )
    .chain_err(|| "Failed to start window")
}
