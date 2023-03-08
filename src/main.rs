use std::fs::*;
use std::path::{Path, PathBuf};

use glam::*;
use sdl2::event::*;
use sdl2::keyboard::*;
use sdl2::messagebox::*;
use serde::{Deserialize, Serialize};
use serde_json as json;

use self::scene::*;

mod audio;
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

        canvas.present();

        game.update(&mut scene);
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
