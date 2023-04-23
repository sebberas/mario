use glam::*;
use sdl2::pixels::*;
use serde::{Deserialize, Serialize};

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
pub struct Enemy {
    pub position: UVec2,
    pub kind: EnemyKind,
    pub is_shown: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum EnemyKind {
    Goomba { from: UVec2, to: UVec2 },
    Piranha { position: UVec2 },
    Koopa { position: UVec2 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Item {
    FireFlower,
    Star,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct Entity {
    pub position: UVec2,
    pub kind: EntityKind,
    pub is_shown: bool,
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
    pub speed: f32,
    pub is_shown: bool,
}

impl ToSprite for Player {
    fn to_sprite(&self) -> Sprite {
        Sprite::new(
            (uvec2(0, 0), uvec2(12, 16)),
            "assets/sprites/mario_test.png",
        )
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

    pub map_tiles: Vec<MapTile>,
    pub background: Rgba,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SceneId(usize);

#[derive(Debug, Deserialize, Serialize)]
pub struct SpriteId(usize);

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Sprite {
    pub bounding_box: (UVec2, UVec2),
    pub asset_path: &'static str,
}

impl Sprite {
    /// Creates a new sprite from a bounding box.
    ///
    /// The coordinates in the second UVec2 is relative to the top-left of the
    /// bounding box.
    pub fn new(bounding_box: (UVec2, UVec2), asset_path: &'static str) -> Sprite {
        Sprite {
            bounding_box,
            asset_path,
        }
    }
}

impl ToSprite for Enemy {
    fn to_sprite(&self) -> Sprite {
        match self.kind {
            EnemyKind::Goomba { .. } => Sprite::new((uvec2(0, 10), uvec2(0, 10)), ""),
            EnemyKind::Piranha { .. } => Sprite::new((uvec2(0, 10), uvec2(0, 10)), ""),
            EnemyKind::Koopa { .. } => Sprite::new((uvec2(0, 10), uvec2(0, 10)), ""),
            _ => unreachable!(),
        }
    }
}

impl ToSprite for Entity {
    fn to_sprite(&self) -> Sprite {
        match self.kind {
            EntityKind::Coin => Sprite::new((uvec2(0, 10), uvec2(0, 10)), ""),
            EntityKind::Pipe { .. } => Sprite::new((uvec2(0, 10), uvec2(0, 10)), ""),
            EntityKind::Item(..) => Sprite::new((uvec2(0, 10), uvec2(0, 10)), ""),
        }
    }
}
