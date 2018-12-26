use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::process;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;

use crate::error::*;
use crate::item::PulledItem;
use crate::item::{Item, ItemFromConfig, TextItem, ThreadStart};
use crate::window;

use yaml_rust::Yaml;

#[derive(Clone)]
pub struct Command {
    command_list: Vec<String>,
    command_output: Arc<Mutex<String>>,
    trigger_show: bool,
}

impl Command {
    #[allow(missing_docs)]
    pub fn new(command_list: Vec<String>, trigger_show: bool) -> Self {
        Command {
            command_list,
            command_output: Arc::new(Mutex::new(String::new())),
            trigger_show,
        }
    }

    fn parse(config: &mut HashMap<String, Yaml>) -> Result<Self> {
        config_get!(command, config, into_string, list);
        config_get!(interpreter, config, into_string);
        config_get!(script, config, into_string);
        config_get!(script_path, config, into_string);
        config_get!(trigger_show, config, as_bool, false);

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

            return Ok(Command::new(command, trigger_show));
        }

        if let Some(interpreter) = interpreter {
            if script.is_some() && script_path.is_some() {
                bail!(ErrorKind::ConfigError(
                    "Only one of script and script_path can be set".into(),
                ));
            }

            if let Some(script_path) = script_path {
                let command = vec![interpreter, script_path];
                return Ok(Command::new(command, trigger_show));
            }

            if let Some(script) = script {
                let command = vec![interpreter, "-c".into(), script];
                return Ok(Command::new(command, trigger_show));
            }
        }

        bail!(ErrorKind::ConfigError("No command specified".into()));
    }

    fn create_command(command_list: Vec<String>) -> Result<process::Command> {
        ensure!(
            !command_list.is_empty(),
            ErrorKind::CommandError("Command is empty".into())
        );
        let mut command_iter = command_list.into_iter();
        let mut command = process::Command::new(command_iter.next().unwrap());
        command.args(command_iter).stdout(process::Stdio::piped());
        Ok(command)
    }
}

impl TextItem for Command {
    fn get_text(&self) -> Result<String> {
        Ok(self.command_output.lock().unwrap().trim().into())
    }
}

#[derive(Clone)]
pub struct PulledCommand {
    command: Command,
    interval: Duration,
}

impl PulledItem for PulledCommand {
    fn pull(
        &self,
        window_command_channel: mpsc::Sender<window::Command>,
    ) -> Result<()>
    {
        trace!("Pulling command");
        let mut command =
            Command::create_command(self.command.command_list.clone())?;
        let output =
            command.output().chain_err(|| "Failed to execute command")?;
        *self.command.command_output.lock().unwrap() =
            String::from_utf8(output.stdout)
                .chain_err(|| "Failed to decode bytes into utf8 string")?;
        if self.command.trigger_show {
            window_command_channel.send(window::Command::Show).unwrap();
        }
        Ok(())
    }

    fn get_interval(&self) -> Duration { self.interval }
}

impl TextItem for PulledCommand {
    fn get_text(&self) -> Result<String> { self.command.get_text() }
}

impl Item for PulledCommand {}

impl ItemFromConfig for PulledCommand {
    fn name() -> &'static str { "pulled-command" }

    fn parse(config: &mut HashMap<String, Yaml>) -> Result<Box<Item>> {
        config_get!(interval_sec, config, as_f64, required);
        Ok(Box::new(PulledCommand {
            command: Command::parse(config)?,
            interval: Duration::from_millis((interval_sec * 1000.0) as u64),
        }))
    }
}

pub type PushedCommand = Command;

impl ThreadStart for PushedCommand {
    fn start(
        &self,
        _window_command_channel: mpsc::Sender<window::Command>,
    ) -> Result<()>
    {
        loop {
            trace!("Starting pushed command");
            let command = Command::create_command(self.command_list.clone())?
                .spawn()
                .chain_err(|| "Failed to start command")?;
            let stdout = BufReader::new(command.stdout.unwrap());
            for line in stdout.lines() {
                *self.command_output.lock().unwrap() =
                    line.chain_err(|| "Failed to read line")?;
            }
        }
    }
}

impl Item for PushedCommand {}

impl ItemFromConfig for PushedCommand {
    fn name() -> &'static str { "pushed-command" }

    fn parse(config: &mut HashMap<String, Yaml>) -> Result<Box<Item>> {
        Ok(Box::new(PushedCommand::parse(config)?))
    }
}
