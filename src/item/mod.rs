mod command;
pub use self::command::Command;

mod text_item;
pub use self::text_item::TextItem;

use std::sync::mpsc;
use std::thread;

use error::*;
use window::{Window, WindowCommand};

pub trait Item: Send + Sync {
    /// Start a thread to handle the item
    fn start(
        &self,
        window_command_channel: mpsc::Sender<WindowCommand>,
    ) -> thread::JoinHandle<Result<()>>;

    /// Draw the item to a window
    fn draw(&self, window: &mut Window) -> Result<()>;
}
