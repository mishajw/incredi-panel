use crate::error::*;
use crate::item::ItemStart;
use crate::window::Command;

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// Item that is pulled at regular intervals
pub trait PulledItem {
    /// Trigger a pull
    fn pull(&self, window_command_channel: mpsc::Sender<Command>)
        -> Result<()>;
    /// Get the interval between pulls
    fn get_interval(&self) -> Duration;
}

impl<T: PulledItem> ItemStart for T {
    fn start(
        &self,
        window_command_channel: mpsc::Sender<Command>,
    ) -> Result<()>
    {
        loop {
            self.pull(window_command_channel.clone())?;
            thread::sleep(self.get_interval());
        }
    }
}
