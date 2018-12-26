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

mod config;
mod error;
mod item;
mod util;
mod window;

quick_main!(run);

fn run() -> error::Result<()> {
    // Initialize logging
    let env = env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, "trace");
    env_logger::Builder::from_env(env).init();

    config::start_window_from_config("incredi.yaml")
}
