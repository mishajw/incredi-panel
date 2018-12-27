//! Contains items in a panel

mod command;
pub use self::command::{Command, PulledCommand, PushedCommand};

mod text_item;
pub use self::text_item::TextItem;

mod pulled;
pub use self::pulled::PulledItem;

use std::collections::HashMap;
use std::sync::mpsc;

use crate::config::yaml_to_hash_map;
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

/// Create a list of items from a configuration
pub fn parse_items(
    config: &mut HashMap<String, Yaml>,
) -> Result<Vec<Box<Item>>> {
    // Get the yaml objects for the items
    config_get!(items, config, into_hash, list);
    let item_yamls = items
        .into_iter()
        .map(Yaml::Hash)
        .map(yaml_to_hash_map)
        .collect::<Result<Vec<_>>>()?;

    // Create the items
    item_yamls
        .into_iter()
        .map(|mut yaml_object| {
            config_get!(name, yaml_object, into_string, required);
            if name == PulledCommand::name() {
                PulledCommand::parse(&mut yaml_object)
            } else if name == PushedCommand::name() {
                PushedCommand::parse(&mut yaml_object)
            } else {
                Err(ErrorKind::ConfigError(format!(
                    "Unrecognized name: {}",
                    name
                ))
                .into())
            }
        })
        .collect()
}
