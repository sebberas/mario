use glam::*;
use sdl2::pixels::*;
use serde::{Deserialize, Serialize};

use crate::map::*;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Rgba(Vec4);

impl From<Vec4> for Rgba {
    fn from(value: Vec4) -> Self {
        Self { 0: value }
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

#[derive(Debug, Deserialize, Serialize)]
pub struct Camera {
    pub position: Vec2,
}

impl Camera {
    pub fn new(position: Vec2) -> Self {
        Self {
            position
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Enemy {
    Goomba(),
    Piranha(),
    Koopa(),
}

pub enum Item {
    FireFlower,
    Star,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Entity {
    Player(),
    Coin(),
    Pipe(),
    Block(),
    Item(),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Text {}

#[derive(Debug, Deserialize, Serialize)]
pub struct Scene {
    pub camera: Camera,

    pub entities: Vec<(Vec2, Entity)>,
    pub enemies: Vec<(Vec2, Enemy)>,

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
    pub asset_path: String,
}

impl Sprite {
    pub fn new(bounding_box: (UVec2, UVec2), asset_path: String) -> Sprite {
        Sprite {
            bounding_box,
            asset_path,
        }
    }
}
