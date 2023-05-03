use std::cell::RefCell;

use glam::*;
use sdl2::pixels::*;
use serde::{Deserialize, Serialize};

use crate::game::BoundingBox;
use crate::map::*;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Rgba(Vec4);

impl From<Vec4> for Rgba {
    fn from(value: Vec4) -> Self {
        Self(value)
    }
}

impl From<Vec3> for Rgba {
    fn from(value: Vec3) -> Self {
        let Vec3 { x, y, z } = value;
        let alpha = 1.0;
        let color = vec4(x, y, z, alpha);
        Self(color)
    }
}

impl From<Rgba> for Color {
    fn from(value: Rgba) -> Self {
        let color: (u8, u8, u8, u8) = (
            (value.0.x * 255.0) as u8,
            (value.0.y * 255.0) as u8,
            (value.0.z * 255.0) as u8,
            (value.0.w * 255.0) as u8,
        );
        Color {
            r: color.0,
            g: color.1,
            b: color.2,
            a: color.3,
        }
    }
}

pub trait ToSprite {
    fn to_sprite(&self) -> Sprite;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Camera {
    pub position: Vec2,
}

impl Camera {
    pub fn new(position: Vec2) -> Self {
        Self { position }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Direction {
    Forward,
    Backward,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Enemy {
    pub position: Vec2,
    pub kind: EnemyKind,
}

impl Enemy {
    pub fn is_goomba(&self) -> bool {
        matches!(self.kind, EnemyKind::Goomba { .. })
    }

    pub fn is_piranha(&self) -> bool {
        matches!(self.kind, EnemyKind::Piranha { .. })
    }

    pub fn is_koopa(&self) -> bool {
        matches!(self.kind, EnemyKind::Koopa { .. })
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum EnemyKind {
    Goomba {
        from: Vec2,
        to: Vec2,
        direction: Direction,
        #[serde(skip)]
        frame: RefCell<u32>,
    },
    Piranha {},
    Koopa {},
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Item {
    Mushroom,
    Flower,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct Entity {
    pub position: UVec2,
    pub kind: EntityKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum EntityKind {
    Coin,
    Pipe { id: usize },
    Item(Item),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Text {}

#[derive(Debug, Deserialize, Serialize)]
pub struct Player {
    pub position: Vec2,
    pub direction: Direction,
    pub move_velocity: f32,
    pub jump_velocity: f32,
    pub fall_velocity: f32,
    pub can_jump: bool,
    frame: RefCell<u32>,
}

impl Player {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            direction: Direction::Forward,
            move_velocity: 0.0,
            jump_velocity: 0.0,
            fall_velocity: 0.0,
            can_jump: true,
            frame: RefCell::new(0),
        }
    }
}

impl ToSprite for Player {
    fn to_sprite(&self) -> Sprite {
        let mut frame = self.frame.borrow_mut();

        let mut sprite = if self.fall_velocity != 0.0 {
            *frame = 0;
            Sprite::new(
                (uvec2(96, 8), uvec2(16, 16)),
                "assets/sprites/characters.png",
                false,
            )
        } else if self.move_velocity != 0.0 {
            match *frame {
                0 | 1 | 2 => {
                    *frame += 1;
                    Sprite::new(
                        (uvec2(56, 8), uvec2(16, 16)),
                        "assets/sprites/characters.png",
                        false,
                    )
                }
                3 | 4 | 5 | 6 | 7 | 8 => {
                    *frame += 1;
                    Sprite::new(
                        (uvec2(20, 8), uvec2(16, 16)),
                        "assets/sprites/characters.png",
                        false,
                    )
                }
                9 | 10 | 11 | 12 | 13 | 14 => {
                    *frame += 1;
                    if *frame == 15 {
                        *frame = 0
                    };

                    Sprite::new(
                        (uvec2(38, 8), uvec2(16, 16)),
                        "assets/sprites/characters.png",
                        false,
                    )
                }
                _ => unreachable!(),
            }
        } else {
            *frame = 0;
            Sprite::new(
                (uvec2(0, 8), uvec2(16, 16)),
                "assets/sprites/characters.png",
                false,
            )
        };

        sprite.mirror = self.direction == Direction::Backward;
        sprite
    }
}

impl Player {
    pub fn collider(&self) -> BoundingBox {
        BoundingBox::new(self.position.x as f32, self.position.y as f32, 16.0, 16.0)
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(bound(deserialize = "'de: 'static"))]
pub struct Scene {
    pub camera: Camera,

    pub entities: Vec<Entity>,
    pub enemies: Vec<Enemy>,
    pub player: Player,

    pub sprites: Vec<Sprite>,
    pub text: Vec<Text>,

    pub tiles: Vec<MapTile>,
    pub background: UVec3,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SceneId(usize);

#[derive(Debug, Deserialize, Serialize)]
pub struct SpriteId(usize);

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Sprite {
    pub bounding_box: (UVec2, UVec2),
    pub asset_path: &'static str,
    pub mirror: bool,
}

impl Sprite {
    /// Creates a new sprite from a bounding box.
    ///
    /// The coordinates in the second UVec2 is relative to the top-left of the
    /// bounding box.
    pub fn new(bounding_box: (UVec2, UVec2), asset_path: &'static str, mirror: bool) -> Sprite {
        Sprite {
            bounding_box,
            asset_path,
            mirror,
        }
    }
}

impl ToSprite for Enemy {
    fn to_sprite(&self) -> Sprite {
        match &self.kind {
            EnemyKind::Goomba {
                frame, direction, ..
            } => {
                let mut frame = frame.borrow_mut();
                match *frame {
                    0..=15 => {
                        *frame += 1;
                        Sprite::new(
                            (uvec2(0, 16), uvec2(16, 16)),
                            "./assets/sprites/enemies.png",
                            direction == &Direction::Forward,
                        )
                    }
                    16..=32 => {
                        *frame += 1;
                        if *frame == 25 {
                            *frame = 0
                        }

                        Sprite::new(
                            (uvec2(18, 16), uvec2(16, 16)),
                            "./assets/sprites/enemies.png",
                            direction == &Direction::Forward,
                        )
                    }
                    _ => unreachable!(),
                }
            }
            EnemyKind::Piranha { .. } => Sprite::new(
                (uvec2(0, 10), uvec2(16, 16)),
                "./assets/sprites/enemies.png",
                false,
            ),
            EnemyKind::Koopa { .. } => Sprite::new(
                (uvec2(0, 10), uvec2(16, 16)),
                "./assets/sprites/enemies.png",
                false,
            ),
            _ => unreachable!(),
        }
    }
}

impl ToSprite for Entity {
    fn to_sprite(&self) -> Sprite {
        match self.kind {
            EntityKind::Coin => Sprite::new((uvec2(0, 10), uvec2(0, 10)), "", false),
            EntityKind::Pipe { .. } => Sprite::new(
                (uvec2(119, 196), uvec2(33, 34)),
                "./assets/sprites/tilesheet.png",
                false,
            ),
            EntityKind::Item(..) => Sprite::new((uvec2(0, 10), uvec2(0, 10)), "", false),
        }
    }
}
