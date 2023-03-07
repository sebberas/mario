use std::time::Duration;

use sdl2::event::*;
use sdl2::keyboard::*;

use self::scene::*;

mod scene;

struct Game {
    scene: Scene,
}

fn main() {
    let sdl = sdl2::init().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();
    let video = sdl.video().unwrap();
    let audio = sdl.audio().unwrap();

    let window = video.window("Super Mario Bros", 800, 600).build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.present();
    }
}
