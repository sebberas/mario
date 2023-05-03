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
use crate::scene;
use crate::scene::*;

pub struct GameSystems {
    pub audio: AudioManager,
}

#[derive(Debug, Clone)]
struct LevelManager {
    levels: Vec<PathBuf>,
    level_names: Vec<String>,
    current: Option<Level>,
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
            if path.is_file() && let Some(extension) = path.extension() && extension.to_str().unwrap() == "level" {
                let file_name = path.file_name().unwrap();
                let file_name = file_name.to_str().unwrap().strip_suffix(".level").unwrap();

                level_names.push(file_name.to_owned());
                levels.push(path);
            }
        }

        Self {
            levels,
            level_names,
            current: None,
        }
    }

    pub fn load(&mut self, name: &str) -> &Level {
        let i = self
            .level_names
            .iter()
            .enumerate()
            .find_map(|(i, level_name)| if level_name == name { Some(i) } else { None })
            .unwrap();

        let level = read_level(&self.levels[i]).unwrap();

        self.current = Some(level);
        self.current.as_ref().unwrap()
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
    fn load_level(&mut self, level_name: &str, scene: &mut Scene) {
        let level = self.level_manager.load(level_name);
        if let Some(start) = level.start {
            self.load_segment(start, scene);
        }
    }

    fn load_segment(&mut self, segment_id: usize, scene: &mut Scene) {
        let level = self
            .level_manager
            .current
            .as_ref()
            .expect("No level is loaded");

        let segment = &level.segments[segment_id];

        scene.enemies = segment.enemies.clone();
        scene.entities = segment.entities.clone();
        scene.tiles = segment.tiles.clone();
        scene.background = segment.background;
    }
}

impl Game {
    const SAVE_PATH: &str = "./assets/save.json";
    const LEVEL_PATH: &str = "./assets/levels/";

    pub fn new(scene: &mut Scene, systems: &GameSystems) -> Self {
        let level_manager = LevelManager::new(&Self::LEVEL_PATH);

        let mut maptiles = vec![];
        for i in 0..10 {
            maptiles.push(MapTile {
                block: Block::Ground,
                coordinate: uvec2(i, 10),
            });
        }

        let level = Level {
            name: "test".to_string(),
            difficulty: Difficulty::Easy,
            start: Some(0),
            segments: vec![
                Segment {
                    spawn: Some(uvec2(20, 20)),
                    enemies: vec![Enemy {
                        position: vec2(64.0, 64.0),
                        kind: EnemyKind::Goomba {
                            from: vec2(20.0, 20.0),
                            to: vec2(40.0, 20.0),
                            direction: Direction::Forward,
                        },
                    }],
                    tiles: maptiles,
                    entities: vec![Entity {
                        kind: EntityKind::Pipe { id: 1 },
                        position: uvec2(120, 192),
                    }],
                    background: uvec3(146, 144, 255),
                },
                Segment {
                    spawn: Some(uvec2(30, 20)),
                    enemies: Vec::default(),
                    tiles: Vec::default(),
                    entities: Vec::default(),
                    background: uvec3(255, 255, 255),
                },
            ],
        };

        write_level(&"./assets/levels/Level 2.level", &level).unwrap();

        let file = File::open("./assets/save.json").ok();
        let state = file.map(|file| json::from_reader(file).unwrap());

        // systems
        //     .audio
        //     .start(&"./assets/audio/tracks/running_about.wav");

        let mut game = Self {
            level_manager,
            state: state.unwrap_or_default(),
        };

        game.load_level("Level 2", scene);
        game
    }

    pub fn update(&mut self, scene: &mut Scene, systems: &GameSystems, keyboard: KeyboardState) {
        self.move_player(scene, &keyboard);

        for Entity { position, kind } in scene.entities.clone() {
            if let EntityKind::Pipe { id } = kind {
                if scene.player.position.x >= position.x as f32
                    && scene.player.position.x < (position.x + 32) as f32
                    && scene.player.position.y + 16.0 == position.y as f32
                    && keyboard.is_scancode_pressed(Scancode::S)
                {
                    self.load_segment(id, scene);
                    break;
                }
            }
        }

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

    pub fn move_player(&mut self, scene: &mut Scene, keyboard: &KeyboardState) {
        let acceleration = 0.01;
        let max_movespeed = 1.0;
        let max_fallspeed = 3.0;
        let max_jumpspeed = 4.0;
        let gravity = 0.02;

        if keyboard.is_scancode_pressed(Scancode::D) {
            if scene.player.move_velocity < max_movespeed {
                scene.player.move_velocity += acceleration;
                // println!("{:?}", scene.player.speed);
            }
            scene.player.position.x += scene.player.move_velocity;
        }

        if keyboard.is_scancode_pressed(Scancode::A) {
            if scene.player.move_velocity < max_movespeed {
                scene.player.move_velocity += acceleration;
            }
            scene.player.position.x -= scene.player.move_velocity;
        }

        if keyboard.is_scancode_pressed(Scancode::Space) && scene.player.can_jump == true {
            scene.player.position.y -= scene.player.jump_velocity;
        }

        let nearby_tiles = Self::nearby_tiles(scene);

        // gravity
        if Self::position_to_coordinate(scene.player.position.y) <= 10 {
            if scene.player.fall_velocity <= max_fallspeed {
                scene.player.fall_velocity += gravity;
            }
            scene.player.position.y += scene.player.fall_velocity;
        } else {
            scene.player.fall_velocity = 0.0;
        }

        println!("fall_velocity {:?}", scene.player.fall_velocity);
    }

    pub fn nearby_tiles(scene: &mut Scene) -> Vec<MapTile> {
        let mut nearby_tiles = vec![];
        let search_distance = 2000.0;
        for block in scene.tiles.iter() {
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

    //takes nearby tiles and create a vec of bounding boxes
    pub fn update_boundingboxes(tiles: Vec<MapTile>) -> Vec<BoundingBox> {
        let mut bounding_boxes = vec![];

        for tile in tiles {
            let x = tile.coordinate.x;
            let y = tile.coordinate.y;
            bounding_boxes.push(BoundingBox::new(x as f32, y as f32, 16.0, 16.0));
        }
        return bounding_boxes;
    }

    pub fn check_collission(self, scene: &mut Scene, tiles: Vec<BoundingBox>) {
        let player_collider = BoundingBox::new(
            scene.player.position.x as f32,
            scene.player.position.y as f32,
            16.0,
            16.0,
        );

        for tile in tiles {
            if Self::collides(player_collider, tile).0 {
                match Self::collides(player_collider, tile).1 {
                    1 => scene.player.move_velocity = 0.0,
                    2 => scene.player.move_velocity = 0.0,
                    3 => scene.player.jump_velocity = 0.0,
                    4 => scene.player.jump_velocity = 0.0,
                    _ => unreachable!(),
                }
            }
        }
    }

    pub fn collides(this: BoundingBox, that: BoundingBox) -> (bool, i32) {
        let dx = (this.x + this.width / 2.0) - (that.x + that.width / 2.0);
        let dy = (this.y + this.height / 2.0) - (that.y + that.height / 2.0);
        let combined_half_widths = this.width / 2.0 + that.width / 2.0;
        let combined_half_heights = this.height / 2.0 + that.height / 2.0;

        if dx.abs() < combined_half_widths && dy.abs() < combined_half_heights {
            let overlap_x = combined_half_widths - dx.abs();
            let overlap_y = combined_half_heights - dy.abs();
            if overlap_x < overlap_y {
                if dx > 0.0 {
                    (true, 1) // Collision on right side
                } else {
                    (true, 2) // Collision on left side
                }
            } else {
                if dy > 0.0 {
                    (true, 3) // Collision on bottom side
                } else {
                    (true, 4) // Collision on top side
                }
            }
        } else {
            (false, 0) // No collision
        }
    }

    pub fn coordinate_to_position(coordinate: u32) -> f32 {
        coordinate as f32 * 16.0
    }

    pub fn position_to_coordinate(position: f32) -> u32 {
        position as u32 / 16
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}
impl BoundingBox {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> BoundingBox {
        BoundingBox {
            x,
            y,
            width,
            height,
        }
    }
}
