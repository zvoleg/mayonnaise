extern crate sdl2;

use sdl2::Sdl;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::EventPump;
use sdl2::pixels::{Color, PixelFormat, PixelFormatEnum};
use sdl2::rect::Rect;

pub struct RecourceHolder {
    sdl: Sdl,
    texture_creator: TextureCreator<WindowContext>,
}

impl RecourceHolder {
    pub fn init(pixel_size: u32) -> (RecourceHolder, Canvas<Window>) {
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();
        let window = video.window("mayonnaise", 256 * pixel_size + 20 + 256, 240 * pixel_size).
            position_centered().
            opengl().
            build().unwrap();
        let canvas = window.into_canvas().accelerated().build().unwrap();
        let texture_creator = canvas.texture_creator();
        (RecourceHolder { sdl, texture_creator }, canvas)
    }
}

pub struct Screen<'a> {
    recource_holder: &'a RecourceHolder,
    canvas: Canvas<Window>,
    main_area: Area<'a>,
    sprite_area: Area<'a>,
    pixel_format: PixelFormat,
}

impl<'a> Screen<'a> {
    pub fn new(recource_holder: &'a RecourceHolder, canvas: Canvas<Window>, pixel_size: u32) -> Screen<'a> {
        let pixel_format = unsafe { PixelFormat::from_ll(sdl2::sys::SDL_AllocFormat(PixelFormatEnum::RGB24 as u32)) };
        let main_texture = recource_holder.texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, 256, 240).
            map_err(|e| e.to_string()).unwrap();
        let main_area = Area::new(main_texture, 0, 0, 256, 240, pixel_size);
        let sprite_texture = recource_holder.texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, 256, 128).
            map_err(|e| e.to_string()).unwrap();
        let sprite_area = Area::new(sprite_texture, (256 * pixel_size + 20) as i32, 0, 256, 128, 1);
        Screen { recource_holder: recource_holder, canvas, main_area, sprite_area, pixel_format }
    }

    pub fn set_point_at_main_area(&mut self, color: u32) {
        self.main_area.set_next_point(Color::from_u32(&self.pixel_format, color));
    }

    pub fn get_events(&mut self) -> EventPump {
        self.recource_holder.sdl.event_pump().unwrap()
    }

    pub fn clear(&mut self) {
        self.canvas.clear();
    }

    pub fn update(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.main_area.update_texture();
        self.sprite_area.update_texture();
        self.canvas.copy(
            &self.main_area.texture,
            None,
            self.main_area.dst).unwrap();
        self.canvas.copy(
            &self.sprite_area.texture,
            None,
            self.sprite_area.dst).unwrap();
        self.canvas.present();
    }
}

struct Area<'a> {
    buffer: Vec<Color>,
    texture: Texture<'a>,
    dst: Option<Rect>,
    idx: usize,
}

impl<'a> Area<'a> {
    fn new(texture: Texture, x: i32, y: i32, width: u32, height: u32, pixel_size: u32) -> Area {
        let dst = Some(Rect::new(x, y, width * pixel_size, height * pixel_size));
        let buffer = vec![Color::RGB(0x99, 0x99, 0x99); (width * height) as usize];
        Area { buffer, texture, dst, idx: 0 }
    }

    fn set_next_point(&mut self, color: Color) {
        self.buffer[self.idx] = color;
        self.idx += 1;
        if self.idx == self.buffer.len() {
            self.idx = 0;
        }
    }

    fn update_texture(&mut self) {
        let buff = &self.buffer;
        self.texture.with_lock(None, |buffer: &mut [u8], _pitch: usize| {
            for (i, color) in buff.iter().enumerate() {
                let (r, g, b) = color.rgb();
                let idx = i * 3;
                buffer[idx] = r;
                buffer[idx + 1] = g;
                buffer[idx + 2] = b;
            }
        }).unwrap();
    }
}
