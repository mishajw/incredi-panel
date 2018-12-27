use crate::error::*;
use crate::item::ItemDraw;
use crate::window::Window;

use sfml::graphics::{Color, Text};

const FONT_SIZE_SCALE: f32 = 1.40;

/// Item that draws text
pub trait TextItem: Send + Sync {
    /// Get text to be drawn
    fn get_text(&self) -> Result<String>;
}

impl<T: TextItem> ItemDraw for T {
    fn draw(&self, window: &mut Window) -> Result<()> {
        let text = self.get_text()?;
        let font = window.config.font.clone();
        let mut sfml_text: Text =
            Text::new(&text, &font, window.config.font_size);
        sfml_text.set_fill_color(&Color::rgb(240, 240, 240));
        sfml_text.set_outline_color(&Color::rgb(10, 10, 10));
        sfml_text.set_outline_thickness(1.0);
        let bounds = sfml_text.global_bounds();

        trace!("Drawing text: \"{}\"", text);
        window.draw(
            vec![&sfml_text],
            (bounds.left + bounds.width) as u32,
            (window.config.font_size as f32 * FONT_SIZE_SCALE) as u32,
        );

        Ok(())
    }
}
