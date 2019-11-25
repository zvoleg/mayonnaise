extern crate sdl2;

use sdl2::Sdl;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

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
}