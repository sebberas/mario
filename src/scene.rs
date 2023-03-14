use glam::*;
use serde::{Deserialize, Serialize};

pub type Rgba = Vec4;

#[derive(Debug, Deserialize, Serialize)]
pub struct Camera(Vec2);

impl Camera {
    pub fn new(position: Vec2) -> Self {
        Self(position)
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
pub struct Tile {
    sprite: SpriteId,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Text {}

#[derive(Debug, Deserialize, Serialize)]
pub struct Scene {
    pub camera: Camera,

    pub entities: Vec<(Vec2, Entity)>,
    pub enemies: Vec<(Vec2, Enemy)>,

    pub text: Vec<Text>,

    pub tiles: Vec<Tile>,
    pub background: Rgba,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SceneId(usize);

#[derive(Debug, Deserialize, Serialize)]
pub struct SpriteId(usize);

#[derive(Debug, Deserialize, Serialize)]
pub struct Sprite {
    pub position: Vec2,
    pub asset_path: String,
    pub size: u32,
}

impl Sprite {
    pub fn new(position: Vec2, asset_path: String, size: u32) -> Sprite {
        Sprite {
            position,
            asset_path,
            size,
        }
    }
}
