//! Handles window setup and drawing using SFML

mod config;
mod grid;

use std::sync::{mpsc, Arc};
use std::thread;
use std::time::Instant;

pub use self::config::Config;
use self::grid::Grid;
use crate::anchor::Anchor;
use crate::dock::dock_window;
use crate::error::*;
use crate::item::Item;
use crate::util;

use sfml::graphics::{
    Color, Drawable, RectangleShape, RenderStates, RenderTarget, RenderWindow,
    Shape, Transform,
};
use sfml::system::{Vector2f, Vector2i};
use sfml::window::{Event, Key, Style, VideoMode};

const WINDOW_NAME: &str = "incredi";
const BORDER_COLOR: (u8, u8, u8) = (50, 50, 50);
const BORDER_THICKNESS: f32 = 3.0;

/// Window where the panel is displayed
pub struct Window {
    /// SFML window used for drawing
    pub sfml_window: RenderWindow,
    /// Configuration for the window
    pub config: Config,
    items: Vec<Arc<Item>>,
    receive: mpsc::Receiver<Command>,
    send: mpsc::Sender<Command>,
    last_shown: Option<Instant>,
    grid: Grid,
}

impl Window {
    #[allow(missing_docs)]
    pub fn start(config: Config, items: Vec<Box<Item>>) -> Result<()> {
        info!("Starting window");

        // Start all the item threads
        let (send, receive) = mpsc::channel::<Command>();
        let items: Vec<Arc<Item>> =
            items.into_iter().map(|i| i.into()).collect();
        items.iter().for_each(|i: &Arc<Item>| {
            let i = i.clone();
            let send = send.clone();
            util::start_thread(move || i.start(send));
        });

        // Set up SFML window
        let mut sfml_window = RenderWindow::new(
            (
                config.grid_width * config.grid_size,
                config.grid_height * config.grid_size,
            ),
            WINDOW_NAME,
            Style::CLOSE,
            &Default::default(),
        );
        sfml_window.set_vertical_sync_enabled(true);
        send.send(Command::Show).unwrap();
        dock_window(WINDOW_NAME)?;

        // Create incredi window object
        let mut window = Window {
            items: items.into_iter().map(|i| i.into()).collect(),
            sfml_window,
            receive,
            send,
            last_shown: None,
            grid: Grid::new(config.grid_width, config.grid_height),
            config,
        };

        window.window_loop()
    }

    /// Draw a list of drawables to the window
    pub fn draw(&mut self, drawables: Vec<&Drawable>, width: u32, height: u32) {
        let grid_width =
            (width as f32 / self.config.grid_size as f32).ceil() as u32;
        let grid_height =
            (height as f32 / self.config.grid_size as f32).ceil() as u32;
        let (grid_x, grid_y) = self.grid.find_space(grid_width, grid_height);

        let x_offset = if self.config.horizontal_centre_align {
            ((grid_width * self.config.grid_size) - width) / 2
        } else {
            0
        };
        let y_offset = if self.config.vertical_centre_align {
            ((grid_height * self.config.grid_size) - height) / 2
        } else {
            0
        };

        let grid_size = self.config.grid_size;
        let create_renderstates = move || {
            let mut default = RenderStates::default();
            let mut transform = Transform::default();
            transform.translate(
                (grid_x * grid_size) as f32,
                (grid_y * grid_size) as f32,
            );
            default.transform = transform;
            default
        };

        {
            // Draw the border of the item
            let mut shape = RectangleShape::with_size(Vector2f::new(
                (grid_width * self.config.grid_size) as f32
                    - BORDER_THICKNESS * 2.0,
                (grid_height * self.config.grid_size) as f32
                    - BORDER_THICKNESS * 2.0,
            ));
            shape.set_fill_color(&Color::rgba(0, 0, 0, 0));
            let (r, g, b) = BORDER_COLOR;
            shape.set_outline_color(&Color::rgb(r, g, b));
            shape.set_outline_thickness(BORDER_THICKNESS);
            let mut renderstates = create_renderstates();
            renderstates
                .transform
                .translate(BORDER_THICKNESS, BORDER_THICKNESS);
            self.sfml_window
                .draw_with_renderstates(&shape, renderstates);
        }

        for drawable in drawables {
            let mut renderstates = create_renderstates();
            renderstates
                .transform
                .translate(x_offset as f32, y_offset as f32);
            self.sfml_window
                .draw_with_renderstates(drawable, renderstates);
        }
    }

    fn window_loop(&mut self) -> Result<()> {
        loop {
            for command in self.get_commands() {
                if self.handle_command(command)? {
                    return Ok(());
                }
            }

            self.draw_items()?;
        }
    }

    fn get_commands(&mut self) -> Vec<Command> {
        let mut result = vec![];
        if self.last_shown.is_none() {
            result.push(self.receive.recv().unwrap());
        }
        while let Some(event) = self.sfml_window.poll_event() {
            result.push(Command::Event(event));
        }
        while let Ok(command) = self.receive.try_recv() {
            result.push(command);
        }
        result
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
                let show_duration = self.config.show_duration;
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
                        < self.config.show_duration
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
        self.grid = Grid::new(self.config.grid_width, self.config.grid_height);
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
        let x = match self.config.anchor {
            Anchor::TopLeft | Anchor::BottomLeft => self.config.edge_distance,
            Anchor::TopRight | Anchor::BottomRight => {
                desktop_mode.width - window_size.x - self.config.edge_distance
            }
        };
        let y = match self.config.anchor {
            Anchor::TopLeft | Anchor::TopRight => self.config.edge_distance,
            Anchor::BottomLeft | Anchor::BottomRight => {
                desktop_mode.height - window_size.y - self.config.edge_distance
            }
        };

        Vector2i::new(x as i32, y as i32)
    }
}

/// Commands that can be sent to the window
#[derive(Clone, Copy)]
pub enum Command {
    /// SFML window event
    Event(Event),
    /// Show the display
    Show,
    /// Hide the display
    Hide,
    /// Quit the program
    Quit,
}
