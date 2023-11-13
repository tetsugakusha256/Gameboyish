use std::time::Instant;

use minifb::{MouseMode, Window, WindowOptions};
use raqote::{DrawOptions, DrawTarget, PathBuilder, SolidSource, Source};

pub const GAMEBOY_SCREEN_WIDTH: usize = 144usize;
pub const GAMEBOY_SCREEN_HEIGHT: usize = 160usize;
pub struct Screen {
    width: usize,
    height: usize,
    window: Option<Window>,
    refresh_rate_delta: usize,
    last_refresh: Instant,
}
impl Screen {
    pub fn new(width: usize, height: usize) -> Screen {
        Screen {
            width,
            height,
            window: None,
            refresh_rate_delta: 16,
            last_refresh: Instant::now(),
        }
    }
    pub fn init(&mut self) {
        let window = Window::new(
            "Raqote",
            self.width,
            self.height,
            WindowOptions {
                // resize: true,
                ..WindowOptions::default()
            },
        )
        .unwrap();
        self.window = Some(window);
    }

    pub fn next_tick(&mut self) {
        if self.last_refresh.elapsed().as_millis() >= self.refresh_rate_delta as u128 {
            println!("refresh: {:?}", self.last_refresh.elapsed());
            self.last_refresh = Instant::now();
            self.draw();
        }
    }
    // pub fn draw(&mut self, array: [[u8;GAMEBOY_SCREEN_WIDTH];GAMEBOY_SCREEN_HEIGHT]) {
    pub fn draw(&mut self) {
        // let vec = array.concat();
        if let Some(window) = &mut self.window {
            let size = window.get_size();
            window
                .update_with_buffer(
                    &random_buffer(GAMEBOY_SCREEN_WIDTH, GAMEBOY_SCREEN_HEIGHT),
                    GAMEBOY_SCREEN_WIDTH,
                    GAMEBOY_SCREEN_HEIGHT,
                )
                .unwrap();
        }
    }
}
fn random_buffer(width: usize, height: usize) -> Vec<u32> {
    let azure_blue = from_u8_rgb(0, 127, 255);

    let t = Instant::now();
    let mut buffer = vec![azure_blue; width * height];
    for ele in &mut buffer {
        *ele = from_u8_rgb((t.elapsed().as_nanos() % 255) as u8, 62, 55);
    }
    buffer
}
fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}
