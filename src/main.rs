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

mod audio;
mod renderer;
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

    let mut sprite_list: Vec<Sprite> = vec![];

    for i in 0..10 {
        sprite_list.push(Sprite {
            position: Vec2::from(((i * 100) as f32, 30.0)),
            asset_path: String::from("assets/sprites/mario_test.png"),
            size: 5,
        })
    }

    let mut renderer = Renderer::new(&mut canvas, &mut sprite_list);

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
        renderer.update();
    }

    game.on_destroy(&mut scene);
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Game {
    completed: Vec<SceneId>,
}

impl Game {
    const SAVE: &str = "./assets/save.json";

    pub fn new(scene: &mut Scene) -> Self {
        let file = File::open("./assets/save.json").ok();

        let game = file.map(|file| json::from_reader(file).unwrap());
        game.unwrap_or_default()
    }

    pub fn update(&mut self, scene: &mut Scene) {}

    pub fn on_destroy(&mut self, scene: &mut Scene) {
        let contents = json::to_string_pretty(self).unwrap();

        if write(Self::SAVE, contents).is_err() {
            let msg = "Due to un unexpected error, the game could not be saved and your progress will be lost.";
            let _ = show_simple_message_box(MessageBoxFlag::ERROR, "Saving Game", msg, None);
        }
    }
}
