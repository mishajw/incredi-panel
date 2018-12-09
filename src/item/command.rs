use std::process;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

use error::*;
use item::TextItem;
use util;
use window::WindowCommand;

pub struct Command {
    command_list: Vec<String>,
    interval: Duration,
    command_output: Arc<Mutex<String>>,
}

impl Command {
    #[allow(missing_docs)]
    pub fn new(command_list: Vec<String>, interval: Duration) -> Self {
        Command {
            command_list,
            interval,
            command_output: Arc::new(Mutex::new(String::new())),
        }
    }

    fn create_command(command_list: Vec<String>) -> Result<process::Command> {
        ensure!(
            !command_list.is_empty(),
            ErrorKind::CommandError("Command is empty".into())
        );
        let mut command_iter = command_list.into_iter();
        let mut command = process::Command::new(command_iter.next().unwrap());
        command.args(command_iter);
        Ok(command)
    }
}

impl TextItem for Command {
    fn get_text(&self) -> Result<String> {
        Ok(self.command_output.lock().unwrap().clone())
    }

    fn start(
        &self,
        redraw_channel: mpsc::Sender<WindowCommand>,
    ) -> thread::JoinHandle<Result<()>>
    {
        let command_list = self.command_list.clone();
        let interval = self.interval;
        let command_output = self.command_output.clone();
        util::start_thread(move || -> Result<()> {
            loop {
                trace!("Executing command");
                let mut command = Self::create_command(command_list.clone())?;
                let output = command
                    .output()
                    .chain_err(|| "Failed to execute command")?;
                *command_output.lock().unwrap() =
                    String::from_utf8(output.stdout).chain_err(|| {
                        "Failed to decode bytes into utf8 string"
                    })?;
                redraw_channel.send(WindowCommand::Show).unwrap();
                thread::sleep(interval);
            }
        })
    }
}
