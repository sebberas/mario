use std::fs::*;
use std::path::{Path, PathBuf};

use glam::*;
use sdl2::event::*;

mod renderer;

use sdl2::keyboard::*;
use serde::{Deserialize, Serialize};
use serde_json as json;

use image::*;

use self::scene::*;

mod scene;

fn main() {
    let sdl = sdl2::init().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();
    let video = sdl.video().unwrap();
    let audio = sdl.audio().unwrap();

    let window = video.window("Super Mario Bros", 800, 600).build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    let mut scene = Scene {
        camera: Camera::new(vec2(0.0, 0.0)),
        enemies: Vec::default(),
        entities: Vec::default(),
        text: Vec::default(),
        tiles: Vec::default(),
        background: vec4(1.0, 1.0, 1.0, 1.0),
    };

    let renderer: Renderer = Renderer::new();

    let game = Game::new(&mut scene);

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

#[derive(Serialize, Deserialize, Debug, Default)]
struct Game {}

impl Game {
    pub fn new(scene: &mut Scene) -> Self {
        // std::fs::write(
        //     "./assets/scenes/scene.json",
        //     json::to_string(&scene).unwrap(),
        // );

        let file = File::open("./assets/save.json").ok();

        file.map(json::from_reader)
            .transpose()
            .unwrap()
            .unwrap_or_default()
    }

    pub fn update(&mut self, scene: &mut Scene) {}

    pub fn on_destroy(&mut self, scene: &mut Scene) {}
}
