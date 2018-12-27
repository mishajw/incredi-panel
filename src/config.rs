//! Utilities for parsing YAML configuration files

use crate::error::*;

use std::collections::HashMap;
use std::fs;
use yaml_rust::{Yaml, YamlLoader};

/// Configurations are a dictionary of value names to YAML values
pub type Config = HashMap<String, Yaml>;

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

/// Get the configuration from a file
pub fn get_config(config_path: &str) -> Result<Config> {
    let yaml = get_yaml(config_path)?;
    yaml_to_hash_map(yaml)
}

/// Convert a yaml object to a hash map
pub fn yaml_to_hash_map(yaml: Yaml) -> Result<Config> {
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
