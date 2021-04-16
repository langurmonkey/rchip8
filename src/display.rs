use crate::constants;

use sdl2::rect::Rect;
use sdl2::{pixels::Color, EventPump};
use sdl2::{render::Canvas, video::Window};

pub struct Display {
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    pub col: Color,
}

impl Display {
    pub fn initialize(window_title: &str) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();
        let window = video_subsystem
            .window(
                window_title,
                (constants::DISPLAY_WIDTH * constants::DISPLAY_SCALE) as u32,
                (constants::DISPLAY_HEIGHT * constants::DISPLAY_SCALE) as u32,
            )
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        Display {
            canvas,
            event_pump,
            col: Color::RGB(200, 200, 200),
        }
    }

    // Clears the display to black
    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.present();
    }

    // Renders the given buffer to the display
    pub fn render(&mut self, buffer: [u8; constants::DISPLAY_LEN]) {
        // Fill with buffer
        self.canvas.set_draw_color(self.col);
        let scl = constants::DISPLAY_SCALE;
        for x in 0..constants::DISPLAY_WIDTH {
            for y in 0..constants::DISPLAY_HEIGHT {
                if buffer[y * constants::DISPLAY_WIDTH + x] != 0 {
                    // Paint!
                    self.canvas.fill_rect(Rect::new(
                        (x * scl) as i32,
                        (y * scl) as i32,
                        scl as u32,
                        scl as u32,
                    ));
                }
            }
        }

        self.canvas.present();
    }
}
