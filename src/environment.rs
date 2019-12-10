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
    main_area: Area,
}

impl Screen {
    pub fn new(pixel_size: u32) -> Screen {
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();
        let window = video.window("mayonnaise", 256 * pixel_size + 20 + 128, 240 * pixel_size).
            position_centered().
            build().unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        let main_area = Area::new(0, 0, 256, 240, pixel_size);
        Screen{ sdl, canvas, main_area }
    }

    pub fn set_point_at_main_area(&mut self, x: i32, y: i32, color: u32) {
        let r = (color >> 16) as u8;
        let g = (color >> 8) as u8;
        let b = color as u8;
        self.main_area.set_point(x, y, Color::RGB(r, g, b), &mut self.canvas)
    }

    pub fn get_events(&mut self) -> EventPump {
        self.sdl.event_pump().unwrap()
    }

    pub fn clear(&mut self) {
        self.canvas.clear();
    }

    pub fn update(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.main_area.draw_border(&mut self.canvas);
        self.set_point_at_main_area(40, 10, 0xFF33AA); // implement point array
        self.canvas.present();
    }
}

struct Area {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    pixel_size: u32
}

impl Area {
    fn new(x: i32, y: i32, width: u32, height: u32, pixel_size: u32) -> Area {
        Area { x, y, width, height, pixel_size }
    }

    fn draw_border(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(0xFF, 0xFF, 0xFF));
        let rect = Rect::new(self.x, self.y, self.width * self.pixel_size, self.height * self.pixel_size);
        canvas.draw_rect(rect).unwrap();
    }

    fn set_point(&self, x: i32, y: i32, color: Color, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(color);
        canvas.fill_rect(Rect::new(self.x + x, self.y + y, self.pixel_size, self.pixel_size)).unwrap();
    }
}
