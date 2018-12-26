mod command;
pub use self::command::{Command, PulledCommand};

mod text_item;
pub use self::text_item::TextItem;

mod pulled;
pub use self::pulled::PulledItem;

use std::collections::HashMap;
use std::sync::mpsc;

use crate::error::*;
use crate::window;

use yaml_rust::Yaml;

pub trait Item: ThreadStart + ItemDraw + Send + Sync {}

/// Start a thread to handle the item
pub trait ThreadStart {
    fn start(
        &self,
        window_command_channel: mpsc::Sender<window::Command>,
    ) -> Result<()>;
}

/// Draw the item to a window
pub trait ItemDraw {
    fn draw(&self, window: &mut window::Window) -> Result<()>;
}

pub trait ItemFromConfig {
    fn name() -> &'static str;

    fn parse(config: &mut HashMap<String, Yaml>) -> Result<Box<Item>>;
}
