use crate::engine::Engine;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

pub struct Interface {
    engine: Engine,
}

impl Interface {
    const WIDTH: u32 = 1024;
    const HEIGHT: u32 = 1024;

    // Constructs a new Interface with a default engine
    pub fn new() -> Self {
        Self {
            engine: Engine::new(),
        }
    }

    // Constructs a new Interface with a user-defined engine
    pub fn with_engine(engine: Engine) -> Self {
        Self { engine }
    }

    pub fn run(&self) {
        let sdl_context = sdl2::init().expect("Failed to load SDL2.");
        let video_subsystem = sdl_context.video().expect("Failed to get video subsystem.");

        let window = video_subsystem
            .window("Tet.rs", Self::WIDTH, Self::HEIGHT)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window
            .into_canvas()
            .build()
            .expect("Failed to build the canvas.");

        let mut event_pump = sdl_context
            .event_pump()
            .expect("Failed to get the event loop.");

        loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => return,
                    _ => {}
                }
            }
            // The rest of the game loop goes here...

            canvas.set_draw_color(Color::RGB(0, 255, 255));
            canvas.clear();
            canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
