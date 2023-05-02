use std::fs::*;
use std::path::{Path, PathBuf};
use std::str::SplitInclusive;

use ::glam::*;
use ::sdl2::keyboard::*;
use ::sdl2::messagebox::*;
use ::serde::{Deserialize, Serialize};
use ::serde_json as json;

use crate::audio::*;
use crate::level::*;
use crate::map::*;
use crate::scene::*;

pub struct GameSystems {
    pub audio: AudioManager,
}

#[derive(Debug, Clone)]
struct LevelManager {
    levels: Vec<PathBuf>,
    level_names: Vec<String>,
}

impl LevelManager {
    pub fn new(path: &impl AsRef<Path>) -> Self {
        let mut levels = Vec::new();
        let mut level_names = Vec::new();

        let directory = read_dir(path).unwrap();
        for entry in directory.flatten() {
            // We consider all files that ends in .level inside of the levels folder a valid
            // level.
            let path = entry.path();
            if path.is_file() && path.ends_with(".level") {
                let file_name = path.file_name().unwrap();
                level_names.push(file_name.to_str().unwrap().to_owned());
                levels.push(path);
            }
        }

        Self {
            levels,
            level_names,
        }
    }

    fn names(&self) -> &[String] {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct GameState {}

#[derive(Debug, Clone)]
pub struct Game {
    level_manager: LevelManager,
    state: GameState,
}

impl Game {
    const SAVE_PATH: &str = "./assets/save.json";
    const LEVEL_PATH: &str = "./assets/levels";

    pub fn new(scene: &mut Scene, systems: &GameSystems) -> Self {
        let level_manager = LevelManager::new(&Self::LEVEL_PATH);

        let file = File::open("./assets/save.json").ok();
        let state = file.map(|file| json::from_reader(file).unwrap());

        systems
            .audio
            .start(&"./assets/audio/tracks/running_about.wav");

        scene.enemies.push(Enemy {
            position: vec2(40f32, 40f32),
            kind: EnemyKind::Goomba {
                from: vec2(2f32, 2f32),
                to: vec2(100f32, 100f32),
                direction: Direction::Forward,
            },
            is_shown: true,
        });

        Self {
            level_manager,
            state: state.unwrap_or_default(),
        }
    }

    pub fn update(&mut self, scene: &mut Scene, systems: &GameSystems, keyboard: KeyboardState) {
        self.move_player(scene, keyboard);

        Self::update_enemies(scene);
    }

    pub fn update_enemies(scene: &mut Scene) {
        let Scene { enemies, .. } = scene;
        // TODO: Multhreaaaaaaaading goooooooo brrrrrrrrrrrrrrrrr
        // Spawn three tasks that take ownership of the different types of enemies.
        // These three tasks handle all logic that is required for that type of enemy
        // except player to enemy collision. Then
        // await those tasks and combine all the new enemies again. Essentially three
        // heap-allocations, but the branch predictor is very happy. Maybe slower, but
        // Steen likey like.

        // Movement
        let goombas: Vec<_> = enemies
            .iter()
            .filter(|enemy| enemy.is_goomba())
            .cloned()
            .collect();

        let goombas = Self::update_goombas(goombas);

        enemies.clear();
        enemies.extend(goombas);

        // Collision

        // Animation State
    }

    pub fn update_goombas(mut goombas: Vec<Enemy>) -> Vec<Enemy> {
        const SPEED: f32 = 0.5;

        for goomba in goombas.iter_mut() {
            let Enemy { position, kind, .. } = goomba;
            let EnemyKind::Goomba {
                from,
                to,
                direction,
            } = kind else { unreachable!() };

            match direction {
                Direction::Forward => position.x += SPEED,
                Direction::Backward => position.x -= SPEED,
            }

            match position.x {
                x if x > to.x => *direction = Direction::Backward,
                x if x < from.x => *direction = Direction::Forward,
                _ => {}
            }
        }

        goombas
    }

    pub fn on_destroy(&mut self, scene: &mut Scene) {
        let contents = json::to_string_pretty(&self.state).unwrap();

        if write(Self::SAVE_PATH, contents).is_err() {
            let msg = "Due to un unexpected error, the game could not be saved and your progress will be lost.";
            let _ = show_simple_message_box(MessageBoxFlag::ERROR, "Saving Game", msg, None);
        }
    }

    pub fn move_player(&mut self, scene: &mut Scene, keyboard: KeyboardState) {
        let acceleration = 0.01;
        let max_speed = 0.5;
        let gravity = 0.5;

        if keyboard.is_scancode_pressed(Scancode::D) {
            if scene.player.speed < max_speed {
                scene.player.speed += acceleration;
                // println!("{:?}", scene.player.speed);
            }
            scene.player.position.x += scene.player.speed;
        }

        if keyboard.is_scancode_pressed(Scancode::A) {
            if scene.player.speed < max_speed {
                scene.player.speed += acceleration;
            }
            scene.player.position.x -= scene.player.speed;
        }

        if keyboard.is_scancode_pressed(Scancode::Space) && scene.player.can_jump == true {
            scene.player.position.y -= scene.player.jump_speed;
        }

        let nearby_tiles = Self::nearby_tiles(scene);

        // gravity
        if Self::position_to_coordinate(scene.player.position.y) <= 10 {
            scene.player.position.y += gravity;
        }
    }

    pub fn nearby_tiles(scene: &mut Scene) -> Vec<MapTile> {
        let mut nearby_tiles = vec![];
        let search_distance = 2000.0;
        for block in scene.map_tiles.iter() {
            // check x distance
            if (block.coordinate.x as f32 - scene.player.position.x).abs() < search_distance
                || (scene.player.position.x - block.coordinate.x as f32).abs() < search_distance
            {
                nearby_tiles.push(*block);
            }

            // check y distance
            if (block.coordinate.y as f32 - scene.player.position.y).abs() < search_distance
                || (scene.player.position.y - block.coordinate.y as f32).abs() < search_distance
            {
                nearby_tiles.push(*block);
            }
        }

        return nearby_tiles;
    }

    pub fn coordinate_to_position(coordinate: u32) -> f32 {
        coordinate as f32 * 16.0
    }

    pub fn position_to_coordinate(position: f32) -> u32 {
        position as u32 / 16
    }
}
