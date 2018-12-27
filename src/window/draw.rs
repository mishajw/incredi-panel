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
}

impl DrawConfig {
    #[allow(missing_docs)]
    pub fn parse(config: &mut Config) -> Result<Self> {
        config_get!(vertical_centre_align, config, into_bool, true);
        config_get!(horizontal_centre_align, config, into_bool, true);
        Ok(DrawConfig {
            vertical_centre_align,
            horizontal_centre_align,
        })
    }
}
