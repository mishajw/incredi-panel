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
pub struct ItemDrawConfig {
    /// The width of the item in pixels
    pub width: u32,
    /// The height of the item in pixels
    pub height: u32,
    /// Whether to verically centre the item
    pub vertical_centre_align: bool,
    /// Whether to horizontally centre the item
    pub horizontal_centre_align: bool,
}

impl ItemDrawConfig {
    #[allow(missing_docs)]
    pub fn new(
        width: u32,
        height: u32,
        vertical_centre_align: bool,
        horizontal_centre_align: bool,
    ) -> Self
    {
        ItemDrawConfig {
            width,
            height,
            vertical_centre_align,
            horizontal_centre_align,
        }
    }
}
