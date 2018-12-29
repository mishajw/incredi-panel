#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate incredi_lib;

use incredi_lib::error::*;
use incredi_lib::ipc;
use incredi_lib::window::Command;

quick_main!(run);

fn run() -> Result<()> {
    // Initialize logging
    let env = env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, "warning");
    env_logger::Builder::from_env(env).init();

    let matches = clap_app!(incredic =>
        (@arg command: -c --command +takes_value +required)
        (@arg fifo_path: -f --fifo-path +takes_value)
    )
    .get_matches();

    let command_str = matches.value_of("command").unwrap();
    let fifo_path = matches
        .value_of("fifo_path")
        .map(str::to_string)
        .unwrap_or(ipc::default_fifo_path());
    let command: Command = command_str.parse()?;
    ipc::send(command, fifo_path)
}
