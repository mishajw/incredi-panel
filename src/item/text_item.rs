use error::*;
use item::Item;
use window::{Command, Window};

use std::sync::mpsc;
use std::thread;

use sfml::graphics::{Color, Text};

pub trait TextItem: Send + Sync {
    fn start(
        &self,
        window_command_channel: mpsc::Sender<Command>,
    ) -> thread::JoinHandle<Result<()>>;
    fn get_text(&self) -> Result<String>;
}

impl<T: TextItem> Item for T {
    fn draw(&self, window: &mut Window) -> Result<()> {
        let text = self.get_text()?;
        let font = window.font.clone();
        let mut sfml_text: Text = Text::new(&text, &font, 24);
        sfml_text.set_fill_color(&Color::RED);
        sfml_text.set_outline_color(&Color::YELLOW);
        sfml_text.set_outline_thickness(2.0);
        let bounds = sfml_text.global_bounds();

        trace!("Drawing text: \"{}\"", text);
        window.draw(
            vec![&sfml_text],
            (bounds.left + bounds.width) as u32,
            (bounds.top + bounds.height) as u32,
        );

        Ok(())
    }

    fn start(
        &self,
        redraw_channel: mpsc::Sender<Command>,
    ) -> thread::JoinHandle<Result<()>>
    {
        <Self as TextItem>::start(self, redraw_channel)
    }
}
