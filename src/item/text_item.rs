use error::*;
use item::Item;
use window::{Command, Window};

use std::sync::mpsc;
use std::thread;

use sfml::graphics::{Color, RenderTarget, Text};

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
        if text.is_empty() {
            return Ok(());
        }

        trace!("Drawing text: {}", text);
        let mut text = Text::new(&text, &window.font, 24);
        text.set_fill_color(&Color::RED);
        text.set_outline_color(&Color::YELLOW);
        text.set_outline_thickness(2.0);
        window.sfml_window.draw(&text);
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
