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

    config::start_window_from_config("incredi.yaml")
}
