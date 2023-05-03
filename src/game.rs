use std::cell::RefCell;
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
use crate::renderer::Renderer;
use crate::scene;
use crate::scene::*;

const GRAVITY: f32 = 9.82 * 0.001;

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

        let mut level = Level {
            name: "test".to_string(),
            difficulty: Difficulty::Easy,
            start: Some(0),
            segments: vec![
                Segment {
                    spawn: Some(uvec2(20, 2)),
                    enemies: vec![
                        Enemy {
                            position: vec2(64.0, 210.0 as f32),
                            kind: EnemyKind::Goomba {
                                from: vec2(20.0, 210.0),
                                to: vec2(100.0, 210.0),
                                direction: Direction::Forward,
                                frame: RefCell::new(0),
                            },
                        },
                        Enemy {
                            position: vec2(128.0, 20.0 as f32),
                            kind: EnemyKind::Koopa {
                                frame: RefCell::new(0),
                            },
                        },
                    ],
                    entities: vec![Entity {
                        kind: EntityKind::Pipe { id: 1 },
                        position: uvec2(120, 192),
                    }],
                    tiles: {
                        let mut tiles = Vec::with_capacity((Renderer::TILES_X * 4) as _);
                        for i in 0..Renderer::TILES_X {
                            for j in 0..2 {
                                let x = i;
                                let y = (Renderer::TILES_Y - 2) + j;

                                tiles.push(MapTile {
                                    block: Block::Ground,
                                    coordinate: uvec2(x, y),
                                })
                            }
                        }
                        for i in 0..4 {
                            tiles.push(MapTile {
<<<<<<< HEAD
                                block: Block::Ground,
                                coordinate: uvec2(10 + i, 12),
                            });
                        }

                        for i in 0..4 {
                            tiles.push(MapTile {
                                block: Block::Ground,
                                coordinate: uvec2(21 + i, 12),
=======
                                block: Block::Wall,
                                coordinate: uvec2(
                                    64 + i * Renderer::TILE_SIZE,
                                    12 * Renderer::TILE_SIZE,
                                ),
>>>>>>> 1fe3cd5edb6f046d7fe5a0265eacf0fa355c2ac4
                            });
                        }

                        tiles
                    },
                    background: uvec3(146, 144, 255),
                },
                Segment {
                    spawn: Some(uvec2(30, 20)),
                    enemies: Vec::default(),
                    tiles: {
                        let mut tiles = Vec::with_capacity((Renderer::TILES_X * 4) as _);
                        for i in 0..Renderer::TILES_X {
                            for j in 0..4 {
                                let x = i;
                                let y = (Renderer::TILES_Y - 5) + j;

                                tiles.push(MapTile {
                                    block: Block::Ground,
                                    coordinate: uvec2(x, y),
                                })
                            }
                        }

                        tiles
                    },
                    entities: Vec::default(),
                    background: uvec3(0, 0, 0),
                },
            ],
        };

        write_level(&"./assets/levels/Level 2.level", &level).unwrap();

        let file = File::open("./assets/save.json").ok();
        let state = file.map(|file| json::from_reader(file).unwrap());

        systems
            .audio
            .start(&"./assets/audio/tracks/running_about.wav");

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
                    && (scene.player.position.y + 16.0) as u32 == position.y
                    && keyboard.is_scancode_pressed(Scancode::S)
                {
                    self.load_segment(id, scene);
                    return;
                }
            }
        }

        Self::update_enemies(scene);
    }

    pub fn update_enemies(scene: &mut Scene) {
        let Scene {
            enemies, player, ..
        } = scene;

        // TODO: Multhreaaaaaaaading goooooooo brrrrrrrrrrrrrrrrr
        // Spawn three tasks that take ownership of the different types of enemies.
        // These three tasks handle all logic that is required for that type of enemy
        // except player to enemy collision. Then
        // await those tasks and combine all the new enemies again. Essentially three
        // heap-allocations, but the branch predictor is very happy. Maybe slower, but
        // Steen likey like.

        // Koopas or Goombas are killed if the head of that enemy is jumped on by the
        // player.

        for enemy in enemies.iter() {
            match player.collider().collides_with(&enemy.collider()) {
                Some(Hit::Left) | Some(Hit::Right) | Some(Hit::Bottom) => {
                    show_simple_message_box(
                        MessageBoxFlag::empty(),
                        "Game Over",
                        "You have died dumb fuck",
                        None,
                    );
                }
                _ => {}
            }
        }

        let enemies: Vec<_> = enemies.clone().into_iter().filter(|enemy| {
            if let Some(hit) = player.collider().collides_with(&enemy.collider()) && hit == Hit::Top {
                false
            } else {
                true
            }
        }).collect();

        // Movement
        let goombas: Vec<_> = enemies
            .iter()
            .filter(|enemy| enemy.is_goomba())
            .cloned()
            .collect();

        let updated_goombas = Self::update_goombas(goombas, scene);

        let koopas: Vec<_> = enemies
            .iter()
            .filter(|enemy| enemy.is_koopa())
            .cloned()
            .collect();

        scene.enemies.clear();
        scene.enemies.extend(updated_goombas);
        scene.enemies.extend(koopas);

        // Collision

        // Animation State
    }

    pub fn update_goombas(mut goombas: Vec<Enemy>, scene: &mut Scene) -> Vec<Enemy> {
        const GOOMBA_SPEED: f32 = 0.2;

        // Update movement
        for goomba in goombas.iter_mut() {
            let Enemy { position, kind } = goomba;
            let EnemyKind::Goomba {
                from,
                to,
                direction,
                ..
            } = kind else { unreachable!() };

            match direction {
                Direction::Forward => *position += vec2(GOOMBA_SPEED, 0.0),
                Direction::Backward => *position -= vec2(GOOMBA_SPEED, 0.0),
            }

            match position.x {
                x if x > to.x => *direction = Direction::Backward,
                x if x < from.x => *direction = Direction::Forward,
                _ => {}
            }

            // Handle gravity
            if let Some(tile_collider) = closest_ground(scene, &scene.tiles.clone()) {
                match goomba.collider().collides_with(&tile_collider) {
                    Some(Hit::Bottom) => break,

                    _ => {}
                }
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
        let move_acceleration = 0.3;
        let max_movespeed = 1.0;
        let max_fallspeed = 2.0;
        let max_jumpspeed = 5.0;
        let gravity_acceleration = 0.03;

        // handle collision
        let nearby_tiles = &scene.tiles.clone();

        // move left and right
        if keyboard.is_scancode_pressed(Scancode::D) {
            scene.player.direction = Direction::Forward;
            if scene.player.move_velocity < max_movespeed {
                scene.player.move_velocity += move_acceleration;
            }
        }

        if keyboard.is_scancode_pressed(Scancode::A) {
            scene.player.direction = Direction::Backward;
            if scene.player.move_velocity > -max_movespeed {
                scene.player.move_velocity -= move_acceleration;
            }
        }

        if !keyboard.is_scancode_pressed(Scancode::D) && !keyboard.is_scancode_pressed(Scancode::A)
        {
            scene.player.move_velocity = 0.0;
        }

        if let Some(collider) = closest_side(scene, &nearby_tiles.clone()) {
            match scene.player.collider().collides_with(&collider) {
                Some(Hit::Left) => {
                    scene.player.move_velocity = -scene.player.move_velocity;
                }
                Some(Hit::Right) => {
                    scene.player.move_velocity = -scene.player.move_velocity;
                }
                Some(Hit::Bottom) => scene.player.jump_velocity = 0.0,
                Some(Hit::Top) => scene.player.jump_velocity = 0.0,
                _ => {}
            }
        }

        if keyboard.is_scancode_pressed(Scancode::Space) && scene.player.can_jump == true {
            scene.player.jump_velocity = max_jumpspeed;
        }
        if scene.player.jump_velocity >= 0.0 {
            scene.player.position.y -= scene.player.jump_velocity;
        }

        if let Some(collider) = closest_ground(scene, &nearby_tiles.clone()) {
            // not falling...
            scene.player.fall_velocity = 0.0;
            scene.player.jump_velocity = 0.0;
            scene.player.can_jump = true;
        } else {
            // in air, accelerate
            if scene.player.fall_velocity <= max_fallspeed {
                scene.player.fall_velocity += gravity_acceleration;
                scene.player.jump_velocity -= scene.player.fall_velocity;
            }
            // update
            scene.player.position.y += scene.player.fall_velocity;
            scene.player.can_jump = false;
        }

        scene.player.position.x += scene.player.move_velocity;
    }
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

pub fn closest_ground(scene: &mut Scene, nearby_tiles: &Vec<MapTile>) -> Option<BoundingBox> {
    let mut closest_tile = None;
    for tile in nearby_tiles.iter() {
        if tile.coordinate.y == (position_to_coordinate(scene.player.position.y) + 1)
            && (tile.coordinate.x == position_to_coordinate(scene.player.position.x)
                || tile.coordinate.x == (position_to_coordinate(scene.player.position.x) + 1))
        {
            let tile_position = (
                coordinate_to_position(tile.coordinate.x),
                coordinate_to_position(tile.coordinate.y),
            );
            closest_tile = Some(BoundingBox::new(
                tile_position.0,
                tile_position.1,
                16.0,
                16.0,
            ))
        }
    }
    return closest_tile;
}

pub fn closest_side(scene: &mut Scene, tiles: &Vec<MapTile>) -> Option<BoundingBox> {
    let Scene { player, .. } = scene;

    for tile in tiles.iter() {
        // println!("{coordinate}");

        let [x, y] = tile.coordinate.to_array();
        if (x == (position_to_coordinate(player.position.x) + 1)
            || x == (position_to_coordinate(player.position.x)))
            && y == position_to_coordinate(player.position.y)
        {
            return Some(tile.collider());
        }
    }

    None
}

pub fn closest_upper() {}

pub fn coordinate_to_position(coordinate: u32) -> f32 {
    coordinate as f32 * 16.0
}

pub fn position_to_coordinate(position: f32) -> u32 {
    position as u32 / 16
}

#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
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
    pub fn collides_with(&self, other: &Self) -> Option<Hit> {
        let dx = (self.x + self.width / 2.0) - (other.x + other.width / 2.0);
        let dy = (self.y + self.height / 2.0) - (other.y + other.height / 2.0);
        let combined_half_widths = self.width / 2.0 + other.width / 2.0;
        let combined_half_heights = self.height / 2.0 + other.height / 2.0;

        if dx.abs() <= combined_half_widths && dy.abs() <= combined_half_heights {
            let overlap_x = combined_half_widths - dx.abs();
            let overlap_y = combined_half_heights - dy.abs();
            if overlap_x < overlap_y {
                if dx > 0.0 {
                    Some(Hit::Right) // Collision on right side
                } else {
                    Some(Hit::Left) // Collision on left side
                }
            } else {
                if dy > 0.0 {
                    Some(Hit::Bottom) // Collision on bottom side
                } else {
                    Some(Hit::Top) // Collision on top side
                }
            }
        } else {
            None // No collision
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hit {
    Top,
    Right,
    Bottom,
    Left,
}

// #[cfg(tests)]
mod tests {
    use glam::uvec2;

    use crate::map::*;

    #[test]
    fn test_collision() {
        let a = MapTile {
            block: Block::Ground,
            coordinate: uvec2(16, 0),
        };

        let b = MapTile {
            block: Block::Ground,
            coordinate: uvec2(16, 0),
        };

        println!("{:?}", a.collider().collides_with(&b.collider()));
    }
}
