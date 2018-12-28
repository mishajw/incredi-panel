//! Floating window "panel". See more
//! [here](https://github.com/mishajw/incredi-panel).

extern crate incredi_lib;
#[macro_use]
extern crate error_chain;

use incredi_lib::{config, error, item, window};

quick_main!(run);

fn run() -> error::Result<()> {
    // Initialize logging
    let env = env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, "warning");
    env_logger::Builder::from_env(env).init();

    // Parse config
    let mut config = config::get_config()?;
    let items = item::parse_items(&mut config)?;
    let window_config = window::Config::parse(&mut config)?;

    // Start window
    window::Window::start(window_config, items)
}
