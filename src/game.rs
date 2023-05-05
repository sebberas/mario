use std::cell::RefCell;
use std::fs::*;
use std::future::join;
use std::path::{Path, PathBuf};
use std::str::SplitInclusive;
use std::time::{Instant, Duration};

use ::glam::*;
use ::sdl2::keyboard::*;
use ::sdl2::messagebox::*;
use ::serde::{Deserialize, Serialize};
use ::serde_json as json;
use futures::executor::ThreadPool;
use futures::future::join3;
use futures::task::{Spawn, SpawnExt};

use crate::audio::*;
use crate::level::*;
use crate::map::*;
use crate::renderer::Renderer;
use crate::scene;
use crate::scene::*;

const GRAVITY: f32 = 9.82 * 0.1;

pub struct GameSystems {
    pub audio: AudioManager,
    pub thread_pool: ThreadPool,
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

    died: Option<Instant>,
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
                            position: vec2(64.0, 210.0),
                            kind: EnemyKind::Goomba {
                                from: vec2(20.0, 210.0),
                                to: vec2(100.0, 210.0),
                                direction: Direction::Forward,
                                frame: RefCell::new(0),
                            },
                        },
                        Enemy {
                            position: vec2(
                                200.0,
                                ((Renderer::TILES_Y - 12) * Renderer::TILE_SIZE) as f32 - 8.0,
                            ),
                            kind: EnemyKind::Koopa {
                                direction: Direction::Forward,
                                shell: None,
                                frame: RefCell::new(0),
                            },
                        },
                        Enemy {
                            position: vec2(104.0 + 8.0, 224.0 - 24.0),
                            kind: EnemyKind::Piranha {
                                frame: RefCell::new(0),
                            },
                        },
                    ],
                    entities: vec![Entity {
                        kind: EntityKind::Pipe { id: 1 },
                        position: uvec2(104, 224),
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
                        // // Stair thingy
                        for i in 0..6 {
                            for j in i..6 {
                                tiles.push(MapTile {
                                    block: Block::Stone,
                                    coordinate: uvec2(j + 10, Renderer::TILES_Y - i - 3),
                                })
                            }
                        }

                        for i in 0..1 {
                            for j in 0..6 {
                                tiles.push(MapTile {
                                    block: Block::Stone,
                                    coordinate: uvec2(i + 16, Renderer::TILES_Y - j - 3),
                                })
                            }
                        }

                        for i in 0..4 {
                            tiles.push(MapTile { block: Block::Ground, coordinate: uvec2(i + 2, 13) });
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

        let mut game = Self {
            level_manager,
            state: state.unwrap_or_default(),
            died: None,
        };

        game.load_level("Level 2", scene);
        game
    }

    pub fn update(&mut self, scene: &mut Scene, systems: &GameSystems, keyboard: KeyboardState) {
        if let Some(died) = self.died {
            let duration = Duration::from_secs(2);
            if died.elapsed() >= duration {
                let buttons = [
                    ButtonData {
                        flags: MessageBoxButtonFlag::NOTHING,
                        button_id: 1,
                        text: "Ohh noo",
                    }
            
                ];
    
               let btn = show_message_box(MessageBoxFlag::empty(), &buttons, "Game Over", "You died, poor loser", None, None).unwrap();
               if let ClickedButton::CustomButton(btn) = btn {
                    if btn.button_id == 1 {
                        panic!("Get good");
                    }
               }
            }


        } else {
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
    
            Self::update_enemies(self, scene, systems);
            if scene.player.position.x > (Renderer::TILES_X * Renderer::TILE_SIZE) as f32 * 0.5 {
                scene.camera.position.x = scene.player.position.x - (Renderer::TILES_X * Renderer::TILE_SIZE) as f32 * 0.5;
            }
        }

    }

    pub fn update_enemies(game: &mut Game, scene: &mut Scene, systems: &GameSystems) {
        let GameSystems { thread_pool, .. } = systems;

        let updated_goombas = {
            let goombas: Vec<_> = scene
                .enemies
                .iter()
                .filter(|el| el.is_goomba())
                .cloned()
                .collect();

            let player = scene.player.clone();
            let tiles = scene.tiles.clone();
            async move { Self::update_goombas(goombas, &player, &tiles) }
        };

        let updated_koopas = {
            let koopas: Vec<_> = scene
                .enemies
                .iter()
                .filter(|enemy| enemy.is_koopa())
                .cloned()
                .collect();

            let player = scene.player.clone();
            let tiles = scene.tiles.clone();
            async move { Self::update_koopas(koopas, &player, &tiles) }
        };

        let updated_piranhas = {
            let piranhas: Vec<_> = scene
                .enemies
                .iter()
                .filter(|enemy| enemy.is_piranha())
                .cloned()
                .collect();

            let player = scene.player.clone();
            let tiles = scene.tiles.clone();
            async move { Self::update_piranhas(piranhas, &player, &tiles) }
        };

        let (goombas, koopas, piranhas) = futures::executor::block_on(join!(
            thread_pool.spawn_with_handle(updated_goombas).unwrap(),
            thread_pool.spawn_with_handle(updated_koopas).unwrap(),
            thread_pool.spawn_with_handle(updated_piranhas).unwrap()
        ));

        scene.enemies.clear();
        scene.enemies.extend(goombas.0);
        scene.enemies.extend(koopas.0);
        scene.enemies.extend(piranhas.0);

        let hits = goombas.1 + koopas.1 + piranhas.1;
        if hits >= 1 {
            systems.audio.start(&"./assets/audio/clips/mariodie.wav");
            game.died = Some(Instant::now());
        }
    }

    pub fn update_goombas(
        goombas: Vec<Enemy>,
        player: &Player,
        tiles: &[MapTile],
    ) -> (Vec<Enemy>, usize) {
        const GOOMBA_SPEED: f32 = 0.2;

        // Handle Dead
        let mut goombas: Vec<_> = goombas
            .into_iter()
            .filter(|goomba| {
                !matches!(
                    goomba.collider().collides_with(&player.collider()),
                    Some((Hit::Bottom, _))
                )
            })
            .collect();

        let mut hits = 0;
        // Update movement
        for goomba in goombas.iter_mut() {
            let EnemyKind::Goomba {
                from,
                to,
                direction,
                ..
            } = &mut goomba.kind else { unreachable!() };

            // Handle Movement
            match direction {
                Direction::Forward => goomba.position += vec2(GOOMBA_SPEED, 0.0),
                Direction::Backward => goomba.position -= vec2(GOOMBA_SPEED, 0.0),
            }

            match goomba.position.x {
                x if x > to.x => *direction = Direction::Backward,
                x if x < from.x => *direction = Direction::Forward,
                _ => {}
            }

            if let Some((hit, _)) = goomba.collider().collides_with(&player.collider()) && hit != Hit::Bottom {
                hits += 1;
            }

            // Handle Gravity
            if let Some(collider) = below_of(goomba.position, uvec2(16, 16), tiles) && let Some((Hit::Top, overlap)) = goomba.collider().collides_with(&collider) {
                goomba.position.y -= overlap;
            } else {
                goomba.position.y += GRAVITY;
            }
        }

        (goombas, hits)
    }

    pub fn update_koopas(
        mut koopas: Vec<Enemy>,
        player: &Player,
        tiles: &[MapTile],
    ) -> (Vec<Enemy>, usize) {
        const KOOPA_SPEED: f32 = 0.15;

        // Handle Dead
        let mut hits = 0;
        for koopa in koopas.iter_mut() {
            // Hide in Shell
            if let Some((Hit::Bottom, _)) = koopa.collider().collides_with(&player.collider()) {
                if let EnemyKind::Koopa { shell, .. } = &mut koopa.kind {
                     *shell = Some(Instant::now()) 
                }
            }

            
            let collider = koopa.collider();
            let EnemyKind::Koopa { direction, shell, .. } = &mut koopa.kind else {unreachable!()};
            
            if shell.is_none() {
                if let Some((hit, _)) = collider.collides_with(&player.collider()) && hit != Hit::Bottom {
                    hits += 1;
                }
            }


            // Gravity
            let size = if shell.is_some() {uvec2(16, 16)} else {uvec2(16, 24)};
            if let Some(collider) = below_of(koopa.position, size, tiles) && let Some((Hit::Top, overlap)) = collider.collides_with(&collider) {
                if overlap != 16.0 {
                        koopa.position.y -= overlap;
                }  
            } else {
                koopa.position.y += GRAVITY;
            }

            if shell.is_none() {
                koopa.position -= vec2(KOOPA_SPEED, 0.0);
            }
        }

        (koopas, hits)
    }

    pub fn update_piranhas(piranhas: Vec<Enemy>, player: &Player, tiles: &[MapTile]) -> (Vec<Enemy>, usize) {
        let mut hits = 0;
        for piranha in &piranhas {
            if let Some((_, _)) = piranha.collider().collides_with(&player.collider()) {
                hits += 1;
            }
        }

        (piranhas, hits)
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
            match scene
                .player
                .collider()
                .collides_with(&collider)
                .map(|e| e.0)
            {
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

/// Returns if there is a tile directly below of `position` in `tiles`.
///
/// If `position` is between two tiles one large bounding box is returned. The
/// bounding box is in world-space.
///
/// # Arguments
///
/// - `position` - The position of the player/entity in world-space.
/// - `tiles` - A slice containing all the tiles that should be checked against
///   (in tile-space).
pub fn below_of(position: Vec2, size: UVec2, tiles: &[MapTile]) -> Option<BoundingBox> {
    let mut left: Option<MapTile> = None;
    let mut right: Option<MapTile> = None;
    for tile in tiles {
        if tile.coordinate == (position + (size.as_vec2())).as_uvec2() / Renderer::TILE_SIZE {
            left = Some(*tile)
        } else if tile.coordinate
            == (position + (size.as_vec2())).as_uvec2() / Renderer::TILE_SIZE - 1
        {
            right = Some(*tile);
        }

        if left.is_some() && right.is_some() {
            break;
        }
    }

    match (left, right) {
        (Some(left), None) => Some(left.collider()),
        (None, Some(right)) => Some(right.collider()),
        (Some(left), Some(right)) => {
            let mut left_collider = left.collider();
            let right_collider = right.collider();

            left_collider.width += right_collider.width;
            left_collider.height = left_collider.height.max(right_collider.height);
            Some(left_collider)
        }

        _ => None,
    }
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

    // en smule overkompliceret når alle colliders er samme størrelse
    pub fn collides_with(&self, other: &Self) -> Option<(Hit, f32)> {
        let dx = (self.x + self.width / 2.0) - (other.x + other.width / 2.0);
        let dy = (self.y + self.height / 2.0) - (other.y + other.height / 2.0);
        let combined_half_widths = self.width / 2.0 + other.width / 2.0;
        let combined_half_heights = self.height / 2.0 + other.height / 2.0;

        if dx.abs() <= combined_half_widths && dy.abs() <= combined_half_heights {
            let overlap_x = combined_half_widths - dx.abs();
            let overlap_y = combined_half_heights - dy.abs();
            if overlap_x < overlap_y {
                if dx > 0.0 {
                    Some((Hit::Right, overlap_x)) // Collision on right side
                } else {
                    Some((Hit::Left, overlap_x)) // Collision on left side
                }
            } else {
                if dy > 0.0 {
                    Some((Hit::Bottom, overlap_y)) // Collision on bottom side
                } else {
                    Some((Hit::Top, overlap_y)) // Collision on top side
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
