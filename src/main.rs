//! Floating window "panel". See more
//! [here](https://github.com/mishajw/incredi-panel).

extern crate incredi_lib;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate clap;

use incredi_lib::{config, error, ipc, item, window};

quick_main!(run);

fn run() -> error::Result<()> {
    // Initialize logging
    let env = env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, "warning");
    env_logger::Builder::from_env(env).init();

    // Parse command line arguments
    let matches = clap_app!(incredi =>
        (@arg fifo_path: -f --fifo-path +takes_value)
    )
    .get_matches();
    let fifo_path = matches
        .value_of("fifo_path")
        .map(str::to_string)
        .unwrap_or(ipc::default_fifo_path());

    // Parse config
    let mut config = config::get_config()?;
    let items = item::parse_items(&mut config)?;
    let window_config = window::Config::parse(&mut config)?;

    // Start window
    window::Window::start(window_config, items, fifo_path)
}
