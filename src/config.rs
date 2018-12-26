use crate::error::*;
use crate::item;
use crate::item::ItemFromConfig;
use crate::window::Window;

use std::collections::HashMap;
use std::fs;
use std::time::Duration;
use yaml_rust::{Yaml, YamlLoader};

macro_rules! config_name {
    ($name:ident) => {
        str::replace(stringify!($name), "_", "-")
    };
}

macro_rules! config_get {
    ($name:ident, $yaml:ident, $get_fn:ident) => {
        let $name = {
            let name_str = config_name!($name);
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
            let name_str = config_name!($name);
            config_get!($name, $yaml, $get_fn);
            $name.ok_or(ErrorKind::ConfigError(format!(
                "Value '{}' does not exist",
                name_str,
            )))?
        };
    };
    ($name:ident, $yaml:ident, $get_fn:ident, list) => {
        let $name = {
            let name_str = config_name!($name);
            config_get!($name, $yaml, into_vec);
            match $name {
                None => vec![],
                Some(array) => array
                    .into_iter()
                    .map(|v| {
                        v.$get_fn().ok_or(
                            ErrorKind::ConfigError(format!(
                                "Failed to get '{}' field using '{}'",
                                name_str,
                                stringify!(get_fn),
                            ))
                            .into(),
                        )
                    })
                    .collect::<Result<_>>()?,
            }
        };
    };
    ($name:ident, $yaml:ident, $get_fn:ident, $default:expr) => {
        let $name = {
            config_get!($name, $yaml, $get_fn);
            $name.unwrap_or($default)
        };
    };
}

pub fn start_window_from_config(config_path: &str) -> Result<()> {
    let yaml = get_yaml(config_path)?;
    let mut yaml_object = get_object(yaml)?;

    config_get!(width, yaml_object, as_i64, 400);
    config_get!(height, yaml_object, as_i64, 200);
    config_get!(show_duration_sec, yaml_object, as_f64, 3.0);
    config_get!(font_path, yaml_object, into_string, required);
    config_get!(font_size, yaml_object, as_i64, 16);
    config_get!(anchor, yaml_object, into_string, "top-right".into());
    config_get!(edge_distance, yaml_object, into_i64, 50);
    config_get!(items, yaml_object, into_hash, list);
    let items = get_items(
        items
            .into_iter()
            .map(Yaml::Hash)
            .map(get_object)
            .collect::<Result<_>>()?,
    )?;

    Window::start(
        width as u32,
        height as u32,
        Duration::from_millis((show_duration_sec * 1000.0) as u64),
        &font_path,
        font_size as u32,
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

fn get_items(
    item_yamls: Vec<HashMap<String, Yaml>>,
) -> Result<Vec<Box<item::Item>>> {
    item_yamls
        .into_iter()
        .map(|mut yaml_object| {
            config_get!(name, yaml_object, into_string, required);
            if name == item::ScheduledCommand::name() {
                item::ScheduledCommand::parse(&mut yaml_object)
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
