mod command;
pub use self::command::Command;

use std::sync::mpsc;
use std::thread;

use error::*;
use window::{Window, WindowCommand};

use sdl2::pixels::Color;

pub trait Item: Send + Sync {
    fn start(
        &self,
        redraw_channel: mpsc::Sender<WindowCommand>,
    ) -> thread::JoinHandle<Result<()>>;
    fn draw(&self, window: &mut Window) -> Result<()>;
}

pub trait TextItem: Send + Sync {
    fn start(
        &self,
        redraw_channel: mpsc::Sender<WindowCommand>,
    ) -> thread::JoinHandle<Result<()>>;
    fn get_text(&self) -> Result<String>;
}

impl<T: TextItem> Item for T {
    fn draw(&self, window: &mut Window) -> Result<()> {
        let text = self.get_text()?;
        if text.is_empty() {
            return Ok(());
        }

        println!("Drawing text: {}", text);
        let font = window
            .font
            .render(&text)
            .blended(Color::RGBA(255, 0, 0, 255))
            .chain_err(|| "Failed to create text")?;

        window
            .canvas
            .set_draw_color(Color::RGBA(195, 217, 255, 255));
        window
            .canvas
            .copy(&font, None, None)
            .chain_err(|| "Failed to write text to window")?;
        window.canvas.clear();
        window.canvas.present();
        Ok(())
    }

    fn start(
        &self,
        redraw_channel: mpsc::Sender<WindowCommand>,
    ) -> thread::JoinHandle<Result<()>>
    {
        <Self as TextItem>::start(self, redraw_channel)
    }
}
