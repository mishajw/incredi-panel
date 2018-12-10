use std::path::Path;
use std::rc::Rc;
use std::sync::mpsc;

use error::*;
use item::Item;

use sdl2::render;
use sdl2::ttf;
use sdl2::video;
use sdl2::Sdl;

const WINDOW_NAME: &str = "incredi";

/// Window where the panel is displayed
pub struct Window<'a> {
    pub context: Sdl,
    pub ttf_context: &'a ttf::Sdl2TtfContext,
    pub font: ttf::Font<'a, 'static>,
    pub texture_creator: render::TextureCreator<video::WindowContext>,
    pub canvas: render::WindowCanvas,
    items: Vec<Rc<Item>>,
}

impl<'a> Window<'a> {
    #[allow(missing_docs)]
    pub fn with_callback(
        width: u32,
        height: u32,
        font_path: &Path,
        items: Vec<Box<Item>>,
    ) -> Result<()>
    {
        println!("Starting window");

        // Mindless window set up
        let context = wrap_sdl_result(sdl2::init())
            .chain_err(|| "Failed to init SDL2")?;
        let video_subsys = wrap_sdl_result(context.video())
            .chain_err(|| "Failed to init video subsys")?;
        let ttf_context =
            sdl2::ttf::init().chain_err(|| "Failed to init TTF context")?;
        let window = video_subsys
            .window(WINDOW_NAME, width, height)
            .position_centered()
            .opengl()
            .build()
            .chain_err(|| "Failed to create window")?;
        let canvas: render::WindowCanvas = window
            .into_canvas()
            .build()
            .chain_err(|| "Failed to create window")?;
        let texture_creator = canvas.texture_creator();

        // Load fonts
        let mut font = wrap_sdl_result(ttf_context.load_font(font_path, 128))
            .chain_err(|| {
            format!(
                "Failed to load font at path {}",
                font_path.to_str().unwrap_or("none")
            )
        })?;
        font.set_style(sdl2::ttf::STYLE_BOLD);

        let (send, receive) = mpsc::channel::<WindowCommand>();
        // Start all the item threads
        // TODO: Handle the join handles
        let _item_handles = items
            .iter()
            .map(|i| i.start(send.clone()))
            .collect::<Vec<_>>();

        let mut window = Window {
            context,
            ttf_context: &ttf_context,
            font,
            texture_creator,
            canvas,
            items: items.into_iter().map(|i| i.into()).collect(),
        };

        window.command_loop(receive)
    }

    fn command_loop(
        &mut self,
        receive: mpsc::Receiver<WindowCommand>,
    ) -> Result<()>
    {
        loop {
            match receive.recv().unwrap() {
                WindowCommand::Show => self.draw()?,
                WindowCommand::Quit => return Ok(()),
            }
        }
    }

    fn draw(&mut self) -> Result<()> {
        println!("Drawing window");
        for mut item in self.items.clone() {
            item.draw(self)?;
        }
        self.canvas.present();
        Ok(())
    }
}

/// Commands that can be sent to the window
pub enum WindowCommand {
    Show,
    #[allow(unused)]
    Quit,
}
