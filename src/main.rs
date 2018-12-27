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
extern crate byteorder;

#[macro_use]
pub mod config;
mod anchor;
mod dock;
pub mod error;
pub mod item;
pub mod util;
pub mod window;
pub use self::anchor::Anchor;
pub use self::dock::dock_window;

quick_main!(run);

fn run() -> error::Result<()> {
    // Initialize logging
    let env = env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, "warning");
    env_logger::Builder::from_env(env).init();

    // Parse config
    let mut config = config::get_config("incredi.yaml")?;
    let items = item::parse_items(&mut config)?;
    let window_config = window::Config::parse(&mut config)?;

    // Start window
    window::Window::start(window_config, items)
}
