//! Sets the panel's window mode to dock. This is not possible using SFML, and
//! therefore has to be done through `xprop`.
//!
//! TODO: Find a better workaround

use crate::error::*;

use std::process::Command;

const WINDOW_TYPE_FIELD: &str = "_NET_WM_WINDOW_TYPE";
const WINDOW_TYPE_VALUE: &str = "_NET_WM_WINDOW_TYPE_DOCK";
const WINDOW_TYPE_FORMAT: &str = "32a";

/// Set the window mode to dock
pub fn dock_window(class_name: &str) -> Result<()> {
    let window_id = get_window_id(class_name)?;
    run_dock_command(window_id)
}

fn get_window_id(class_name: &str) -> Result<u32> {
    let output = Command::new("xdotool")
        .args(&["search", "-classname", class_name])
        .output()
        .chain_err(|| "Failed to get window ID. Is xdotool installed?")?;
    ensure!(
        output.status.success(),
        ErrorKind::CommandError("Command to get window ID failed".into())
    );
    String::from_utf8(output.stdout)
        .chain_err(|| "Failed to decode window ID as UTF8")?
        .trim()
        .parse()
        .chain_err(|| "Failed to parse window ID as integer")
}

fn run_dock_command(window_id: u32) -> Result<()> {
    let status = Command::new("xprop")
        .args(&[
            "-f",
            WINDOW_TYPE_FIELD,
            WINDOW_TYPE_FORMAT,
            "-set",
            WINDOW_TYPE_FIELD,
            WINDOW_TYPE_VALUE,
            "-id",
            &window_id.to_string(),
        ])
        .status()
        .chain_err(|| "Falied to dock window")?;
    ensure!(
        status.success(),
        ErrorKind::CommandError("Command to dock window failed".into())
    );
    Ok(())
}
