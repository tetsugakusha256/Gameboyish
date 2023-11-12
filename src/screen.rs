use std::time::Instant;

use minifb::{MouseMode, Window, WindowOptions};
use raqote::{DrawOptions, DrawTarget, PathBuilder, SolidSource, Source};

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
            refresh_rate_delta: 32,
            last_refresh: Instant::now(),
        }
    }
    pub fn init(&mut self) {
        let window = Window::new(
            "Raqote",
            self.width,
            self.height,
            WindowOptions {
                resize: true,
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
    pub fn draw(&mut self) {
        if let Some(window) = &mut self.window {
            let size = window.get_size();
            let mut dt = DrawTarget::new(size.0 as i32, size.1 as i32);
            dt.clear(SolidSource::from_unpremultiplied_argb(
                0xff, 0xff, 0xff, 0xff,
            ));
            let mut pb = PathBuilder::new();
            if let Some(pos) = window.get_mouse_pos(MouseMode::Clamp) {
                pb.rect(pos.0, pos.1, 100., 130.);
                let path = pb.finish();
                dt.fill(
                    &path,
                    &Source::Solid(SolidSource::from_unpremultiplied_argb(0xff, 0, 0xff, 0)),
                    &DrawOptions::new(),
                );

                window
                    .update_with_buffer(dt.get_data(), size.0, size.1)
                    .unwrap();
            }
        }
    }
}
