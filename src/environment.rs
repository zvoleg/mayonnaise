extern crate sdl2;

use sdl2::Sdl;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::pixels::{Color, PixelFormat, PixelFormatEnum};
use sdl2::rect::Rect;

pub struct Screen {
    sdl: Sdl,
    canvas: Canvas<Window>,
    main_area: Area,
    sprite_area: Area,
}

impl Screen {
    pub fn new(pixel_size: u32) -> Screen {
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();
        let window = video.window("mayonnaise", 256 * pixel_size + 20 + 128, 240 * pixel_size).
            position_centered().
            opengl().
            build().unwrap();
        let canvas = window.into_canvas().build().unwrap();
        let main_area = Area::new(0, 0, 256, 240, pixel_size);
        let sprite_area = Area::new((256 * pixel_size + 20) as i32, 0, 60, 60, 1);
        Screen{ sdl, canvas, main_area, sprite_area }
    }

    pub fn set_point_at_main_area(&mut self, x: i32, y: i32, color: u32) {
        let pixel_format = unsafe { PixelFormat::from_ll(sdl2::sys::SDL_AllocFormat(PixelFormatEnum::RGB24 as u32)) };
        self.main_area.set_point(x, y, Color::from_u32(&pixel_format, color));
    }

    pub fn get_events(&mut self) -> EventPump {
        self.sdl.event_pump().unwrap()
    }

    pub fn clear(&mut self) {
        self.canvas.clear();
    }

    pub fn update(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        let creator = self.canvas.texture_creator();
        let mut main_texture = creator.create_texture_streaming(PixelFormatEnum::RGB24, self.main_area.width, self.main_area.height).
            map_err(|e| e.to_string()).unwrap();
        self.main_area.present(&mut main_texture);
        let mut sprite_texture = creator.create_texture_streaming(PixelFormatEnum::RGB24, self.sprite_area.width, self.sprite_area.height).
            map_err(|e| e.to_string()).unwrap();
        self.sprite_area.present(&mut sprite_texture);
        self.canvas.clear();
        self.canvas.copy(
            &main_texture,
            None,
            self.main_area.dst).unwrap();
        self.canvas.copy(
            &sprite_texture,
            None,
            self.sprite_area.dst).unwrap();
        self.canvas.present();
    }
}

struct Area {
    width: u32,
    height: u32,
    buff: Vec<Color>,
    dst: Option<Rect>
}

impl Area {
    fn new(x: i32, y: i32, width: u32, height: u32, pixel_size: u32) -> Area {
        let buff = vec![Color::RGB(0x99, 0x99, 0x99); (width * height) as usize];
        let dst = Some(Rect::new(x, y, width * pixel_size, height * pixel_size));
        Area { width, height, buff, dst }
    }

    fn set_point(&mut self, x: i32, y: i32, color: Color) {
        let idx = y * self.width as i32 + x;
        self.buff[idx as usize] = color;
    }

    fn present(&self, texture: &mut Texture) {
        texture.with_lock(None, |buffer: &mut [u8], _pitch: usize| {
            for (i, color) in self.buff.iter().enumerate() {
                let (r, g, b) = color.rgb();
                let idx = i * 3;
                buffer[idx] = r;
                buffer[idx + 1] = g;
                buffer[idx + 2] = b;
            }
        }).map_err(|e| e.to_string()).unwrap();
    }
}
