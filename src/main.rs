use std::fs::*;
use std::path::{Path, PathBuf};
use std::str::SplitInclusive;

use glam::*;
use renderer::Renderer;
use sdl2::{event::*, render};

use sdl2::keyboard::*;
use sdl2::messagebox::*;
use serde::{Deserialize, Serialize};
use serde_json as json;

use self::scene::*;
use self::map::*;

mod audio;
mod map;
mod renderer;
mod scene;
mod game;
mod input_handler;

fn main() {
    let sdl = sdl2::init().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();
    let video = sdl.video().unwrap();
    let audio = sdl.audio().unwrap();

    let window = video.window("Super Mario Bros", 800, 600).build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.default_pixel_format();
    canvas.set_blend_mode(sdl2::render::BlendMode::Blend);

    let mut scene = Scene {
        camera: Camera::new(vec2(0.0, 0.0)),
        enemies: Vec::default(),
        entities: Vec::default(),
        sprites: vec![Sprite::new(
            (uvec2(0, 0), uvec2(16, 16)),
            String::from("assets/sprites/mario_test.png"),
        )],
        text: Vec::default(),
        map_tiles: Vec::default(),
        background: vec4(0.0, 1.0, 1.0, 0.0).into(),
    };

    let mut renderer = Renderer::new(&mut canvas);

    let mut game = Game::new(&mut scene);

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
        game.update(&mut scene);
        renderer.update(&mut scene);
    }

    game.on_destroy(&mut scene);
}


