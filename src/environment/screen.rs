extern crate spriter;

use spriter::window::Window;
use spriter::window::Canvas;

use std::rc::Rc;
use std::cell::RefCell;

pub struct Screen {
    main_area: Area,
    sprite_area_left: Area,
    sprite_area_right: Area,
    main_color: Area,
    background_pallettes: Vec<Area>,
    sprite_pallettes: Vec<Area>,
}

impl Screen {
    pub fn new(window: &mut Window, pixel_size: u32, ) -> Screen {
        let main_screen_width = 256;
        let main_screen_height = 240;
        let pattern_tabel_width = 128;

        let main_canvas = window.create_canvas(
            0,
            0,
            main_screen_width * pixel_size,
            main_screen_height * pixel_size,
            main_screen_width,
            main_screen_height);
        let main_area = Area::new(main_canvas, main_screen_width, main_screen_height);

        let sprite_area_left_canvas = window.create_canvas(
            main_screen_width * pixel_size + 20,
            0,
            pattern_tabel_width * pixel_size,
            pattern_tabel_width * pixel_size,
            pattern_tabel_width,
            pattern_tabel_width);
        let sprite_area_left = Area::new(sprite_area_left_canvas, pattern_tabel_width, pattern_tabel_width);

        let sprite_area_right_canvas = window.create_canvas(
            (main_screen_width + pattern_tabel_width) * pixel_size + 22,
            0,
            pattern_tabel_width * pixel_size,
            pattern_tabel_width * pixel_size,
            pattern_tabel_width,
            pattern_tabel_width);
            let sprite_area_right = Area::new(sprite_area_right_canvas, pattern_tabel_width, pattern_tabel_width);

        let main_color_canvas = window.create_canvas(
            main_screen_width * pixel_size + 20,
            pattern_tabel_width * pixel_size + 20,
            10 * pixel_size,
            10 * pixel_size,
            1,
            1);
        let main_color = Area::new(main_color_canvas, 1, 1);

        let background_pallettes: Vec<Area> = (0..4).map(|i| {
            let background_pallette_canvas = window.create_canvas(
                main_screen_width * pixel_size + 20,
                pattern_tabel_width * pixel_size + 20 + 11 * pixel_size * (1 + i),
                40 * pixel_size,
                10 * pixel_size,
                4,
                1);
            return Area::new(background_pallette_canvas, 4, 1);
        }).collect();

        let sprite_pallettes: Vec<Area> = (0..4).map(|i| {
            let sprite_pallette_canvas = window.create_canvas(
                main_screen_width * pixel_size + 20 + 41 * pixel_size,
                pattern_tabel_width * pixel_size + 20 + 11 * pixel_size * (1 + i),
                40 * pixel_size,
                10 * pixel_size,
                4,
                1);
            return Area::new(sprite_pallette_canvas, 4, 1);
        }).collect();

        Screen {
            main_area,
            sprite_area_left,
            sprite_area_right,
            main_color,
            background_pallettes,
            sprite_pallettes
         }
    }

    pub fn set_point_at_main_area(&mut self, color: u32) {
        self.main_area.set_next_point(color);
    }

    pub fn set_point_at_sprite_area(&mut self, color: u32, table: u8) {
        match table {
            0 => self.sprite_area_left.set_next_point(color),
            1 => self.sprite_area_right.set_next_point(color),
            _ => (),
        }
    }

    pub fn set_point_at_main_color_area(&mut self, color: u32) {
        self.main_color.set_next_point(color);
    }

    pub fn set_point_at_background_color_area(&mut self, pallette_id: usize, color: u32) {
        self.background_pallettes[pallette_id].set_next_point(color);
    }

    pub fn set_point_at_sprite_color_area(&mut self, pallette_id: usize, color: u32) {
        self.sprite_pallettes[pallette_id].set_next_point(color);
    }
}

struct Area {
    canvas: Rc<RefCell<Canvas>>,
    width: usize,
    height: usize,
    x: usize,
    y: usize,
}

impl Area {
    fn new(canvas: Rc<RefCell<Canvas>>, width: u32, height: u32) -> Area {
        let width = width as usize;
        let height = height as usize;
        Area { canvas, width, height, x: 0, y: 0 }
    }

    fn set_next_point(&mut self, color: u32) {
        self.canvas.borrow_mut().set_color(self.x, self.y, color);
        self.x += 1;
        if self.x == self.width {
            self.x = 0;
            self.y += 1;
        }
        if self.y == self.height {
            self.y = 0;
        }
    }
}
