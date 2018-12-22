use std::rc::Rc;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use error::*;
use item::Item;

use sfml::graphics::{Color, Font, RenderTarget, RenderWindow};
use sfml::window::{Event, Key, Style};

const WINDOW_NAME: &str = "incredi";

/// Window where the panel is displayed
pub struct Window {
    /// SFML window used for drawing
    pub sfml_window: RenderWindow,
    /// The font used for drawing text
    pub font: Font,
    items: Vec<Rc<Item>>,
    show_duration: Duration,
    receive: mpsc::Receiver<WindowCommand>,
    send: mpsc::Sender<WindowCommand>,
}

impl Window {
    #[allow(missing_docs)]
    pub fn start(
        width: u32,
        height: u32,
        show_duration: Duration,
        font_path: &str,
        items: Vec<Box<Item>>,
    ) -> Result<()>
    {
        info!("Starting window");

        // Start all the item threads
        let (send, receive) = mpsc::channel::<WindowCommand>();
        // TODO: Handle the join handles
        let _item_handles = items
            .iter()
            .map(|i| i.start(send.clone()))
            .collect::<Vec<_>>();

        // Set up SFML window
        let font =
            Font::from_file(font_path).chain_err(|| "Failed to load font")?;
        let mut sfml_window = RenderWindow::new(
            (width, height),
            WINDOW_NAME,
            Style::CLOSE,
            &Default::default(),
        );
        sfml_window.set_vertical_sync_enabled(true);

        // Create incredi window object
        let mut window = Window {
            items: items.into_iter().map(|i| i.into()).collect(),
            font,
            sfml_window,
            show_duration,
            receive,
            send,
        };

        window.window_loop()
    }

    fn window_loop(&mut self) -> Result<()> {
        loop {
            while let Some(event) = self.sfml_window.poll_event() {
                if self.handle_command(WindowCommand::Event(event))? {
                    return Ok(());
                }
            }
            while let Ok(command) = self.receive.try_recv() {
                if self.handle_command(command)? {
                    return Ok(());
                }
            }
            self.draw()?;
        }
    }

    /// Handle a command. Return true if the window should quit
    fn handle_command(&mut self, command: WindowCommand) -> Result<bool> {
        match command {
            WindowCommand::Event(event) => return self.handle_event(event),
            WindowCommand::Show => {
                self.sfml_window.set_visible(true);
                let show_duration = self.show_duration;
                let send = self.send.clone();
                thread::spawn(move || {
                    thread::sleep(show_duration);
                    send.send(WindowCommand::Hide).unwrap();
                });
            }
            WindowCommand::Hide => self.sfml_window.set_visible(false),
            WindowCommand::Quit => {
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn handle_event(&mut self, event: Event) -> Result<bool> {
        match event {
            Event::Closed
            | Event::KeyPressed {
                code: Key::Escape, ..
            } => Ok(true),
            _ => Ok(false),
        }
    }

    fn draw(&mut self) -> Result<()> {
        trace!("Drawing window");
        self.sfml_window.clear(&Color::BLACK);
        for mut item in self.items.clone() {
            item.draw(self)?;
        }
        self.sfml_window.display();
        Ok(())
    }
}

/// Commands that can be sent to the window
#[derive(Clone, Copy)]
pub enum WindowCommand {
    Event(Event),
    Show,
    Hide,
    #[allow(unused)]
    Quit,
}
