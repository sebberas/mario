use std::fs::*;
use std::path::{Path, PathBuf};
use std::str::SplitInclusive;

use glam::vec2;
use sdl2::event::*;
use sdl2::keyboard::*;
use sdl2::messagebox::*;
use sdl2::render;
use serde::{Deserialize, Serialize};
use serde_json as json;

use crate::scene::*;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Game {}

impl Game {
    const SAVE: &str = "./assets/save.json";

    pub fn new(scene: &mut Scene) -> Self {
        let file = File::open("./assets/save.json").ok();

        let game = file.map(|file| json::from_reader(file).unwrap());
        game.unwrap_or_default()
    }

    pub fn update(&mut self, scene: &mut Scene, keyboard: sdl2::keyboard::KeyboardState) {
        self.move_player(scene, keyboard);
    }

    pub fn on_destroy(&mut self, scene: &mut Scene) {
        let contents = json::to_string_pretty(self).unwrap();

        if write(Self::SAVE, contents).is_err() {
            let msg = "Due to un unexpected error, the game could not be saved and your progress will be lost.";
            let _ = show_simple_message_box(MessageBoxFlag::ERROR, "Saving Game", msg, None);
        }
    }

    pub fn move_player(&mut self, scene: &mut Scene, keyboard: sdl2::keyboard::KeyboardState) {
        let acceleration = 0.01;
        let max_speed = 0.1;

        if keyboard.is_scancode_pressed(Scancode::D) {
            if scene.player.speed < max_speed {
                scene.player.speed += acceleration;
                println!("{:?}", scene.player.speed);
            }
            scene.player.position.x += scene.player.speed;
        }

        if keyboard.is_scancode_pressed(Scancode::A) {
            if scene.player.speed < max_speed {
                scene.player.speed += acceleration;
            }
            scene.player.position.x -= scene.player.speed;
        }

        if keyboard.is_scancode_pressed(Scancode::Space) {}
    }
}
