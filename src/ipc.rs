//! Functions for communicating between a panel and CLI

use crate::error::*;
use crate::util;
use crate::window::Command;

use std::env;
use std::fs::{remove_file, File, OpenOptions};
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::mpsc;

use nix::sys::stat::Mode;
use nix::unistd::mkfifo;

const FIFO_NAME: &str = "incredi-fifo";

/// Get the default path for the IPC FIFO
pub fn default_fifo_path() -> String {
    let tmp = env::var("TMPDIR").unwrap_or("/tmp".into());
    match env::var("DISPLAY") {
        Ok(display) => format!("{}/{}{}", tmp, FIFO_NAME, display),
        _ => format!("{}/{}", tmp, FIFO_NAME),
    }
}

/// Start a thread for listening for IPC calls
pub fn spawn_listen_thread(
    command_channel: mpsc::Sender<Command>,
    fifo_path_str: String,
)
{
    util::start_thread(move || listen(command_channel, fifo_path_str));
}

fn listen(
    command_channel: mpsc::Sender<Command>,
    fifo_path_str: String,
) -> Result<()>
{
    let fifo_path = Path::new(&fifo_path_str);
    if fifo_path.exists() {
        warn!("IPC FIFO path exists, recreating");
        remove_file(fifo_path)
            .chain_err(|| "Failed to remove existing IPC FIFO")?;
    }

    mkfifo(fifo_path, Mode::S_IRWXU)
        .chain_err(|| "Failed to create IPC FIFO")?;
    info!("Created IPC fifo at {}", fifo_path_str);

    loop {
        let file =
            File::open(fifo_path).chain_err(|| "Failed to open IPC FIFO")?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line =
                line.chain_err(|| "Failed to read line from IPC FIFO")?;
            let command: Command = line.trim().parse()?;
            debug!("Received IPC command: {}", command);
            command_channel.send(command).unwrap();
        }
    }
}

/// Send an IPC call
pub fn send(command: Command, fifo_path_str: String) -> Result<()> {
    debug!("Opening IPC FIFO");
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&fifo_path_str)
        .chain_err(|| "Failed to open IPC FIFO for writing")?;
    debug!("Writing to IPC FIFO");
    file.write_all(&command.to_string().into_bytes())
        .chain_err(|| "Failed to write to IPC FIFO")
}
