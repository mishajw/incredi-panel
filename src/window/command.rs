use crate::error::*;

use std::fmt;
use std::str::FromStr;

use sfml;

/// Commands that can be sent to the window
#[derive(Clone, Copy)]
pub enum Command {
    /// SFML window event
    Event(sfml::window::Event),
    /// Show the display
    Show,
    /// Hide the display
    Hide,
    /// Quit the program
    Quit,
}

impl FromStr for Command {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "show" => Ok(Command::Show),
            "hide" => Ok(Command::Hide),
            "quit" => Ok(Command::Quit),
            s => bail!(ErrorKind::ConfigError(format!(
                "Unrecognised command: {}",
                s
            ))),
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Command::Show => "show",
            Command::Hide => "hide",
            Command::Quit => "quit",
            Command::Event(_) => "event",
        };
        write!(f, "{}", s)
    }
}
