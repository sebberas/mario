use glam::*;

pub type Rgba = Vec4;

pub struct Camera {}

pub enum Enemy {
    Goomba(),
    Piranha(),
    Koopa(),
}

pub enum Item {
    FireFlower,

    Star,
}

pub enum Entity {
    Player(),
    Coin(),
    Pipe(),
    Block(),
    Item(),
}

pub struct Tile {}

pub struct Text {}

pub struct Scene {
    camera: Camera,

    entities: Vec<Entity>,
    enemies: Vec<Enemy>,

    tiles: Vec<Tile>,
    background: Rgba,

    text: Vec<Text>,
}
