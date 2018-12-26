use std::rc::Rc;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use crate::error::*;
use crate::item::Item;

use sfml::graphics::{
    Color, Drawable, Font, RectangleShape, RenderStates, RenderTarget,
    RenderWindow, Shape, Transform,
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
    /// The font used for drawing text
    pub font: Rc<Font>,
    /// Size of the font
    pub font_size: u32,
    grid_width: u32,
    grid_height: u32,
    grid_size: u32,
    items: Vec<Rc<Item>>,
    show_duration: Duration,
    receive: mpsc::Receiver<Command>,
    send: mpsc::Sender<Command>,
    anchor: Anchor,
    edge_distance: u32,
    last_shown: Option<Instant>,
    grid: Grid,
}

impl Window {
    #[allow(missing_docs)]
    pub fn start(
        grid_width: u32,
        grid_height: u32,
        show_duration: Duration,
        font_path: &str,
        font_size: u32,
        anchor: Anchor,
        edge_distance: u32,
        grid_size: u32,
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
            (grid_width * grid_size, grid_height * grid_size),
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
            grid_width,
            grid_height,
            grid_size,
            font_size,
            sfml_window,
            show_duration,
            receive,
            send,
            anchor,
            edge_distance,
            last_shown: None,
            grid: Grid::new(grid_width, grid_height),
        };

        window.window_loop()
    }

    pub fn draw(&mut self, drawables: Vec<&Drawable>, width: u32, height: u32) {
        let grid_width = (width as f32 / self.grid_size as f32).ceil() as u32;
        let grid_height = (height as f32 / self.grid_size as f32).ceil() as u32;
        let (grid_x, grid_y) = self.grid.find_space(grid_width, grid_height);

        let grid_size = self.grid_size;
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
            let mut shape = RectangleShape::with_size(Vector2f::new(
                (grid_width * self.grid_size) as f32,
                (grid_height * self.grid_size) as f32,
            ));
            shape.set_fill_color(&Color::rgba(0, 0, 0, 0));
            let (r, g, b) = BORDER_COLOR;
            shape.set_outline_color(&Color::rgb(r, g, b));
            shape.set_outline_thickness(BORDER_THICKNESS);
            self.sfml_window
                .draw_with_renderstates(&shape, create_renderstates());
        }
        for drawable in drawables {
            self.sfml_window
                .draw_with_renderstates(drawable, create_renderstates());
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
        self.grid = Grid::new(self.grid_width, self.grid_height);
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

struct Grid {
    width: u32,
    height: u32,
    filled: Vec<u32>,
}

impl Grid {
    fn new(width: u32, height: u32) -> Self {
        Grid {
            width,
            height,
            filled: vec![],
        }
    }

    fn find_space(&mut self, width: u32, height: u32) -> (u32, u32) {
        let width = width.min(self.width);
        let height = height.min(self.height);
        let mut start_index: Option<u32> = None;
        let mut max_column: u32 = 0;
        for (i, f) in self.filled.iter().enumerate() {
            let i = i as u32;
            let space_left = self.width - f;
            match (start_index, space_left >= width) {
                (Some(si), true) => {
                    max_column = max_column.max(*f);
                    if i - si > height {
                        self.fill(max_column, si, width, height);
                        return (max_column, si);
                    }
                }
                (Some(_), false) => {
                    start_index = None;
                    max_column = 0
                }
                (None, true) => start_index = Some(i),
                (None, false) => {}
            }
        }

        let space_y = self.filled.len() as u32;
        self.filled.append(&mut vec![0; height as usize]);
        self.fill(0, space_y, width, height);
        (0, space_y)
    }

    fn fill(&mut self, x: u32, y: u32, width: u32, height: u32) {
        for i in y..(y + height) {
            assert!(self.filled[i as usize] <= x);
            self.filled[i as usize] = x + width;
        }
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
