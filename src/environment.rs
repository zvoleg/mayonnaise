extern crate sdl2;

use sdl2::Sdl;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use self::sdl2::pixels::Color;

pub struct Screen {
    sdl: Sdl,
    canvas: Canvas<Window>,
}

impl Screen {
    pub fn new() -> Screen {
        let mut sdl = sdl2::init().unwrap();
        let mut video = sdl.video().unwrap();
        let mut window = video.window("mayonnaise", 1024, 512).
            position_centered().
            build().unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        Screen{ sdl, canvas }
    }

    pub fn get_events(&mut self) -> EventPump {
        self.sdl.event_pump().unwrap()
    }

    pub fn print_text(&mut self) {
        let ttf_context = self::sdl2::ttf::init().unwrap();
        let mut font = ttf_context.load_font("recources/fonts/ka1.ttf", 14).unwrap();
        let surface = font.render("Hello world").blended(Color::RGB(255, 255, 255)).unwrap();
        let texture_creator = self.canvas.texture_creator();
        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        self.canvas.copy(&texture,None,None).unwrap();
        self.canvas.present();


    }
}