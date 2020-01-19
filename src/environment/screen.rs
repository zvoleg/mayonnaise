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
        let window = video.window("mayonnaise", 256 * pixel_size + 20 + (128 * pixel_size * 2) + 2, 240 * pixel_size).
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
    sprite_area_left: Area<'a>,
    sprite_area_right: Area<'a>,
    main_color: Area<'a>,
    background_pallettes: Vec<Area<'a>>,
    sprite_pallettes: Vec<Area<'a>>,
    pixel_format: PixelFormat,
}

impl<'a> Screen<'a> {
    pub fn new(recource_holder: &'a RecourceHolder, canvas: Canvas<Window>, pixel_size: u32) -> Screen<'a> {
        let main_screen_width = 256;
        let main_screen_height = 240;
        let pattern_tabel_width = 128;

        let pixel_format = unsafe { PixelFormat::from_ll(sdl2::sys::SDL_AllocFormat(PixelFormatEnum::RGB24 as u32)) };

        let main_texture = recource_holder.
            texture_creator.
            create_texture_streaming(
                PixelFormatEnum::RGB24,
                main_screen_width,
                main_screen_height).
            unwrap();
        let main_area = Area::new(main_texture, 0, 0, main_screen_width, main_screen_height, pixel_size);

        let sprite_texture_left = recource_holder.
            texture_creator.
            create_texture_streaming(
                PixelFormatEnum::RGB24,
                pattern_tabel_width,
                pattern_tabel_width).
            unwrap();
        let sprite_area_left = Area::new(
            sprite_texture_left,
            (main_screen_width * pixel_size + 20) as i32,
            0,
            pattern_tabel_width,
            pattern_tabel_width,
            pixel_size);

        let sprite_texture_right = recource_holder.
            texture_creator.
            create_texture_streaming(
                PixelFormatEnum::RGB24,
                pattern_tabel_width,
                pattern_tabel_width).
            unwrap();
        let sprite_area_right = Area::new(
            sprite_texture_right,
            ((main_screen_width + pattern_tabel_width) * pixel_size + 22) as i32,
            0,
            pattern_tabel_width,
            pattern_tabel_width,
            pixel_size);

        let main_color_texture = recource_holder.
            texture_creator.
            create_texture_streaming(
                PixelFormatEnum::RGB24,
                1,
                1).
            unwrap();
        let main_color = Area::new(
            main_color_texture,
            (main_screen_width * pixel_size + 20) as i32,
            (pattern_tabel_width * pixel_size + 20) as i32,
            1,
            1,
            15);

        let background_pallettes: Vec<Area<'a>> = (0..4).map(|i| {
            let background_pallette_texture = recource_holder.
                texture_creator.
                create_texture_streaming(
                    PixelFormatEnum::RGB24,
                    4,
                    1).
            unwrap();
            return Area::new(
                background_pallette_texture,
                (main_screen_width * pixel_size + 20) as i32,
                (pattern_tabel_width * pixel_size + 40 + 20 * i) as i32,
                4,
                1,
                15);
        }).collect();

        let sprite_pallettes: Vec<Area<'a>> = (0..4).map(|i| {
            let sprite_pallette_texture = recource_holder.
            texture_creator.
            create_texture_streaming(
                PixelFormatEnum::RGB24,
                4,
                1).
            unwrap();
            return Area::new(
                sprite_pallette_texture,
                (main_screen_width * pixel_size + 100) as i32,
                (pattern_tabel_width * pixel_size + 40 + 20 * i) as i32,
                4,
                1,
                15);
        }).collect();

        Screen {
            recource_holder,
            canvas,
            main_area,
            sprite_area_left,
            sprite_area_right,
            pixel_format,
            main_color,
            background_pallettes,
            sprite_pallettes
         }
    }

    pub fn set_point_at_main_area(&mut self, color: u32) {
        self.main_area.set_next_point(Color::from_u32(&self.pixel_format, color));
    }

    pub fn set_point_at_sprite_area(&mut self, color: u32, table: u8) {
        match table {
            0 => self.sprite_area_left.set_next_point(Color::from_u32(&self.pixel_format, color)),
            1 => self.sprite_area_right.set_next_point(Color::from_u32(&self.pixel_format, color)),
            _ => (),
        }
    }

    pub fn set_point_at_main_color_area(&mut self, color: u32) {
        self.main_color.set_next_point(Color::from_u32(&self.pixel_format, color));
    }

    pub fn set_point_at_background_color_area(&mut self, pallette_id: usize, color: u32) {
        self.background_pallettes[pallette_id].set_next_point(Color::from_u32(&self.pixel_format, color));
    }

    pub fn set_point_at_sprite_color_area(&mut self, pallette_id: usize, color: u32) {
        self.sprite_pallettes[pallette_id].set_next_point(Color::from_u32(&self.pixel_format, color));
    }

    pub fn get_events(&mut self) -> EventPump {
        self.recource_holder.sdl.event_pump().unwrap()
    }

    pub fn clear(&mut self) {
        self.canvas.clear();
    }

    pub fn update(&mut self) {
        self.canvas.set_draw_color(Color::RGB(50, 50, 50));
        self.canvas.clear();
        Screen::update_canvas(&mut self.canvas, &mut self.main_area);
        Screen::update_canvas(&mut self.canvas, &mut self.sprite_area_left);
        Screen::update_canvas(&mut self.canvas, &mut self.sprite_area_right);
        Screen::update_canvas(&mut self.canvas, &mut self.main_color);
        (0..4).for_each(|i| {
            Screen::update_canvas(&mut self.canvas, &mut self.background_pallettes[i])
        });
        (0..4).for_each(|i| {
            Screen::update_canvas(&mut self.canvas, &mut self.sprite_pallettes[i])
        });
        self.canvas.present();
    }

    fn update_canvas(canvas: &mut  Canvas<Window>, area: &mut Area<'a>) {
        area.update_texture();
        canvas.copy(
            &area.texture,
            None,
            area.dst
        ).unwrap();
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
                let (b, g, r) = color.rgb();
                let idx = i * 3;
                buffer[idx] = r;
                buffer[idx + 1] = g;
                buffer[idx + 2] = b;
            }
        }).unwrap();
    }
}
