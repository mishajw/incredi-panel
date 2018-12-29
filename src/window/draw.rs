use crate::config::Config;
use crate::error::*;

use sfml::graphics::{Drawable, RenderStates};

/// Configure how to draw a `Drawable` item
pub struct DrawableConfig<'a, 'b, 'c, 'd> {
    /// The drawable to draw
    pub drawable: &'a Drawable,
    /// Transformations to apply to the drawable
    pub render_states: RenderStates<'b, 'c, 'd>,
}

impl<'a, 'b, 'c, 'd> DrawableConfig<'a, 'b, 'c, 'd> {
    #[allow(missing_docs)]
    pub fn new(drawable: &'a Drawable) -> Self {
        DrawableConfig {
            drawable,
            render_states: RenderStates::default(),
        }
    }
}

/// Configure how to draw an item
#[derive(Clone)]
pub struct DrawConfig {
    /// Whether to verically centre the item
    pub vertical_centre_align: bool,
    /// Whether to horizontally centre the item
    pub horizontal_centre_align: bool,
    /// Vertical padding in pixels
    pub vertical_padding: u32,
    /// Horizontal padding in pixels
    pub horizontal_padding: u32,
}

impl DrawConfig {
    #[allow(missing_docs)]
    pub fn parse(config: &mut Config) -> Result<Self> {
        config_get!(vertical_centre_align, config, into_bool, true);
        config_get!(horizontal_centre_align, config, into_bool, true);
        config_get!(vertical_padding, config, into_i64, 5);
        config_get!(horizontal_padding, config, into_i64, 10);
        Ok(DrawConfig {
            vertical_centre_align,
            horizontal_centre_align,
            vertical_padding: vertical_padding as u32,
            horizontal_padding: horizontal_padding as u32,
        })
    }
}
