extern crate sdl2;

use sdl2::Sdl;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub struct Screen {
    sdl: Sdl,
    canvas: Canvas<Window>,
    pixel_size: u8,
}

impl Screen {
    pub fn new(pixel_size: u8) -> Screen {
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();
        let window = video.window("mayonnaise", 1024, 512).
            position_centered().
            build().unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(0xFF, 0xFF, 0xFF));
        canvas.draw_rect(Rect::new(0, 0, 256 * pixel_size as u32, 240 * pixel_size as u32)).unwrap();
        canvas.present();
        Screen{ sdl, canvas, pixel_size: 2 }
    }

    pub fn get_events(&mut self) -> EventPump {
        self.sdl.event_pump().unwrap()
    }

    pub fn set_point(&mut self, x: i32, y: i32, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.draw_rect(Rect::new(x, y, self.pixel_size as u32, self.pixel_size as u32)).unwrap();
    }

    pub fn clear(&mut self) {
        self.canvas.clear();
    }

    pub fn update(&mut self) {
        self.canvas.present();
    }
}
