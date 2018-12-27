//! Contains items in a panel

mod command;
pub use self::command::{Command, PulledCommand, PushedCommand};

mod text_item;
pub use self::text_item::TextItem;

mod pulled;
pub use self::pulled::PulledItem;

use std::collections::HashMap;
use std::sync::mpsc;

use crate::error::*;
use crate::window;

use yaml_rust::Yaml;

/// Implementors can be shown on the panel
pub trait Item: ItemStart + ItemDraw + Send + Sync {}

/// Can be started, with the assumption it never terminates
pub trait ItemStart {
    #[allow(missing_docs)]
    fn start(
        &self,
        window_command_channel: mpsc::Sender<window::Command>,
    ) -> Result<()>;
}

/// Draw the item to a window
pub trait ItemDraw {
    #[allow(missing_docs)]
    fn draw(&self, window: &mut window::Window) -> Result<()>;
}

/// Implementors can be created from the configuration
pub trait ItemFromConfig {
    /// The name of the item, matched against the `name` field in the config
    fn name() -> &'static str;

    /// Create the item from the config
    fn parse(config: &mut HashMap<String, Yaml>) -> Result<Box<Item>>;
}
