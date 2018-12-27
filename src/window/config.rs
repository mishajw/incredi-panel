use crate::anchor::Anchor;
use crate::error::*;

use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;

use sfml::graphics::Font;
use yaml_rust::Yaml;

/// Configuration for a window
#[allow(missing_docs)]
pub struct Config {
    pub grid_width: u32,
    pub grid_height: u32,
    pub grid_size: u32,
    pub font: Rc<Font>,
    pub font_size: u32,
    pub show_duration: Duration,
    pub anchor: Anchor,
    pub edge_distance: u32,
}

impl Config {
    #[allow(missing_docs)]
    pub fn parse(yaml_object: &mut HashMap<String, Yaml>) -> Result<Self> {
        config_get!(grid_width, yaml_object, as_i64, 15);
        config_get!(grid_height, yaml_object, as_i64, 10);
        config_get!(grid_size, yaml_object, into_i64, 17);
        config_get!(show_duration_sec, yaml_object, as_f64, 3.0);
        config_get!(font_path, yaml_object, into_string, required);
        config_get!(font_size, yaml_object, as_i64, 16);
        config_get!(anchor, yaml_object, into_string, "top-right".into());
        config_get!(edge_distance, yaml_object, into_i64, 50);
        let font = Rc::new(
            Font::from_file(&font_path).chain_err(|| "Failed to load font")?,
        );

        Ok(Config {
            grid_width: grid_width as u32,
            grid_height: grid_height as u32,
            grid_size: grid_size as u32,
            font,
            font_size: font_size as u32,
            show_duration: Duration::from_millis(
                (show_duration_sec * 1000.0) as u64,
            ),
            anchor: anchor.parse()?,
            edge_distance: edge_distance as u32,
        })
    }
}
