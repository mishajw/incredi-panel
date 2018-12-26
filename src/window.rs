use std::rc::Rc;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use crate::error::*;
use crate::item::Item;

use sfml::graphics::{Color, Drawable, Font, RenderTarget, RenderWindow};
use sfml::system::Vector2i;
use sfml::window::{Event, Key, Style, VideoMode};

const WINDOW_NAME: &str = "incredi";

/// Window where the panel is displayed
pub struct Window {
    /// SFML window used for drawing
    pub sfml_window: RenderWindow,
    /// The font used for drawing text
    pub font: Rc<Font>,
    /// Size of the font
    pub font_size: u32,
    items: Vec<Rc<Item>>,
    show_duration: Duration,
    receive: mpsc::Receiver<Command>,
    send: mpsc::Sender<Command>,
    anchor: Anchor,
    edge_distance: u32,
    last_shown: Option<Instant>,
}

impl Window {
    #[allow(missing_docs)]
    pub fn start(
        width: u32,
        height: u32,
        show_duration: Duration,
        font_path: &str,
        font_size: u32,
        anchor: Anchor,
        edge_distance: u32,
        items: Vec<Box<Item>>,
    ) -> Result<()>
    {
        info!("Starting window");

        // Start all the item threads
        let (send, receive) = mpsc::channel::<Command>();
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
        send.send(Command::Show).unwrap();

        // Create incredi window object
        let mut window = Window {
            items: items.into_iter().map(|i| i.into()).collect(),
            font: Rc::new(font),
            font_size,
            sfml_window,
            show_duration,
            receive,
            send,
            anchor,
            edge_distance,
            last_shown: None,
        };

        window.window_loop()
    }

    pub fn draw(&mut self, drawables: Vec<&Drawable>, width: u32, height: u32) {
        for drawable in drawables {
            self.sfml_window.draw(drawable);
        }
    }

    fn window_loop(&mut self) -> Result<()> {
        loop {
            while let Some(event) = self.sfml_window.poll_event() {
                if self.handle_command(Command::Event(event))? {
                    return Ok(());
                }
            }
            while let Ok(command) = self.receive.try_recv() {
                if self.handle_command(command)? {
                    return Ok(());
                }
            }
            self.draw_items()?;
        }
    }

    /// Handle a command. Return true if the window should quit
    fn handle_command(&mut self, command: Command) -> Result<bool> {
        match command {
            Command::Event(event) => return self.handle_event(event),
            Command::Show => {
                debug!("Showing window");
                self.sfml_window.set_visible(true);
                self.last_shown = Some(Instant::now());
                let window_location = self.get_window_location();
                self.sfml_window.set_position(&window_location);
                let show_duration = self.show_duration;
                let send = self.send.clone();
                thread::spawn(move || {
                    thread::sleep(show_duration);
                    send.send(Command::Hide).unwrap();
                });
            }
            Command::Hide => {
                debug!("Hiding window");
                if self.last_shown.is_some()
                    && Instant::now().duration_since(self.last_shown.unwrap())
                        < self.show_duration
                {
                    debug!("Window not visible for long enough");
                    return Ok(false);
                }
                self.sfml_window.set_visible(false);
                self.last_shown = None;
            }
            Command::Quit => {
                info!("Quitting due to window command");
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

    fn draw_items(&mut self) -> Result<()> {
        trace!("Drawing window");
        self.sfml_window.clear(&Color::BLACK);
        for item in self.items.clone() {
            item.draw(self)?;
        }
        self.sfml_window.display();
        Ok(())
    }

    fn get_window_location(&self) -> Vector2i {
        let desktop_mode = VideoMode::desktop_mode();
        let window_size = self.sfml_window.size();
        let x = match self.anchor {
            Anchor::TopLeft | Anchor::BottomLeft => self.edge_distance,
            Anchor::TopRight | Anchor::BottomRight => {
                desktop_mode.width - window_size.x - self.edge_distance
            }
        };
        let y = match self.anchor {
            Anchor::TopLeft | Anchor::TopRight => self.edge_distance,
            Anchor::BottomLeft | Anchor::BottomRight => {
                desktop_mode.height - window_size.y - self.edge_distance
            }
        };

        Vector2i::new(x as i32, y as i32)
    }
}

/// Commands that can be sent to the window
#[derive(Clone, Copy)]
pub enum Command {
    Event(Event),
    Show,
    Hide,
    #[allow(unused)]
    Quit,
}

/// Where to anchor the panel
#[allow(unused)]
pub enum Anchor {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl std::str::FromStr for Anchor {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "top-left" => Anchor::TopLeft,
            "top-right" => Anchor::TopRight,
            "bottom-left" => Anchor::BottomLeft,
            "bottom-right" => Anchor::BottomRight,
            s => {
                return Err(ErrorKind::ConfigError(format!(
                    "Uncrecognized anchor: {}",
                    s
                ))
                .into());
            }
        })
    }
}
