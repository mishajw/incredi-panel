use std::collections::HashMap;
use std::process;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::error::*;
use crate::item::{Item, ItemFromConfig, TextItem};
use crate::util;
use crate::window::Command;

use yaml_rust::Yaml;

pub struct ScheduledCommand {
    command_list: Vec<String>,
    interval: Duration,
    command_output: Arc<Mutex<String>>,
}

impl ScheduledCommand {
    #[allow(missing_docs)]
    pub fn new(command_list: Vec<String>, interval: Duration) -> Self {
        ScheduledCommand {
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

impl TextItem for ScheduledCommand {
    fn get_text(&self) -> Result<String> {
        Ok(self.command_output.lock().unwrap().clone())
    }

    fn start(
        &self,
        redraw_channel: mpsc::Sender<Command>,
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
                redraw_channel.send(Command::Show).unwrap();
                thread::sleep(interval);
            }
        })
    }
}

impl ItemFromConfig for ScheduledCommand {
    fn name() -> &'static str { "scheduled-command" }

    fn parse(config: &mut HashMap<String, Yaml>) -> Result<Box<Item>> {
        config_get!(command, config, into_string, list);
        config_get!(interpreter, config, into_string);
        config_get!(script, config, into_string);
        config_get!(script_path, config, into_string);
        config_get!(interval_sec, config, into_f64, required);
        let interval = Duration::from_millis((interval_sec * 1000.0) as u64);

        if !command.is_empty() {
            if interpreter.is_some()
                || script.is_some()
                || script_path.is_some()
            {
                bail!(ErrorKind::ConfigError(
                    "If command is set, interpreter, script, and script_path \
                     must not be"
                        .into(),
                ));
            }

            return Ok(Box::new(ScheduledCommand::new(command, interval)));
        }

        if let Some(interpreter) = interpreter {
            if script.is_some() && script_path.is_some() {
                bail!(ErrorKind::ConfigError(
                    "Only one of script and script_path can be set".into(),
                ));
            }

            if let Some(script_path) = script_path {
                let command = vec![interpreter, script_path];
                return Ok(Box::new(ScheduledCommand::new(command, interval)));
            }

            if let Some(script) = script {
                let command = vec![interpreter, "-c".into(), script];
                return Ok(Box::new(ScheduledCommand::new(command, interval)));
            }
        }

        bail!(ErrorKind::ConfigError("No command specified".into()));
    }
}
