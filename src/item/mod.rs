mod scheduled_command;
pub use self::scheduled_command::ScheduledCommand;

mod text_item;
pub use self::text_item::TextItem;

use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

use crate::error::*;
use crate::window::{Command, Window};

use yaml_rust::Yaml;

pub trait Item: Send + Sync {
    /// Start a thread to handle the item
    fn start(
        &self,
        window_command_channel: mpsc::Sender<Command>,
    ) -> thread::JoinHandle<Result<()>>;

    /// Draw the item to a window
    fn draw(&self, window: &mut Window) -> Result<()>;
}

pub trait ItemFromConfig {
    fn name() -> &'static str;

    fn parse(config: &mut HashMap<String, Yaml>) -> Result<Box<Item>>;
}
