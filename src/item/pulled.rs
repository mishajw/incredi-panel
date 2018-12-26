use crate::error::*;
use crate::item::ThreadStart;
use crate::window::Command;

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub trait PulledItem {
    fn pull(&self, window_command_channel: mpsc::Sender<Command>)
        -> Result<()>;
    fn get_interval(&self) -> Duration;
}

impl<T: PulledItem> ThreadStart for T {
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
