use error::*;
use item;
use window::Window;

use std::collections::HashMap;
use std::fs;
use std::time::Duration;
use yaml_rust::{Yaml, YamlLoader};

macro_rules! parse {
    ($name:ident, $yaml:ident, $get_fn:ident) => {
        let $name = {
            let name_str = str::replace(stringify!($name), "_", "-");
            let yaml_value = $yaml.remove(&name_str);
            match yaml_value {
                None => None,
                Some(yaml_value) => {
                    let get_fn_str = stringify!($get_fn);
                    let value = yaml_value.$get_fn().ok_or(
                        ErrorKind::ConfigError(format!(
                            "Failed to get '{}' field using '{}'",
                            name_str, get_fn_str,
                        )),
                    )?;
                    Some(value)
                }
            }
        };
    };
    ($name:ident, $yaml:ident, $get_fn:ident, required) => {
        let $name = {
            parse!($name, $yaml, $get_fn);
            let name_str = str::replace(stringify!($name), "_", "-");
            $name.ok_or(ErrorKind::ConfigError(format!(
                "Value '{}' does not exist",
                name_str,
            )))?
        };
    };
    ($name:ident, $yaml:ident, $get_fn:ident, $default:expr) => {
        let $name = {
            parse!($name, $yaml, $get_fn);
            $name.unwrap_or($default)
        };
    };
}

pub fn start_window_from_config(config_path: &str) -> Result<()> {
    let yaml = get_yaml(config_path)?;
    let mut yaml_object = get_object(yaml)?;

    parse!(width, yaml_object, as_i64, 400);
    parse!(height, yaml_object, as_i64, 200);
    parse!(show_duration_sec, yaml_object, as_f64, 3.0);
    parse!(font_path, yaml_object, into_string, required);
    parse!(anchor, yaml_object, into_string, "top-right".into());
    parse!(edge_distance, yaml_object, into_i64, 50);

    let items: Vec<Box<item::Item>> =
        vec![Box::new(item::ScheduledCommand::new(
            vec!["echo".into(), "-n".into(), "hello".into()],
            Duration::from_secs(5),
        ))];

    Window::start(
        width as u32,
        height as u32,
        Duration::from_millis((show_duration_sec * 1000.0) as u64),
        &font_path,
        anchor.parse()?,
        edge_distance as u32,
        items,
    )
}

fn get_yaml(config_path: &str) -> Result<Yaml> {
    let yaml_str = fs::read_to_string(config_path)
        .chain_err(|| "Failed to read config file")?;
    let mut yaml_list = YamlLoader::load_from_str(&yaml_str)
        .chain_err(|| "Failed to parse YAML")?;
    if yaml_list.is_empty() {
        return Err(ErrorKind::ConfigError("Yaml was empty".into()).into());
    }
    if yaml_list.len() < 1 {
        return Err(ErrorKind::ConfigError(
            "Expected yaml object, got yaml list".into(),
        )
        .into());
    }
    Ok(yaml_list.remove(0))
}

fn get_object(yaml: Yaml) -> Result<HashMap<String, Yaml>> {
    yaml.into_hash()
        .ok_or(ErrorKind::ConfigError("Expected object".into()))?
        .into_iter()
        .map(|(key, value)| {
            let key: String = key
                .as_str()
                .ok_or(ErrorKind::ConfigError(
                    "Found non-string key in yaml".into(),
                ))?
                .into();
            Ok((key, value))
        })
        .collect::<Result<_>>()
}
