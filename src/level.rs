use glam::*;
use serde::{Deserialize, Serialize};

use crate::map::*;
use crate::scene::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Level {
    pub name: String,
    pub difficulty: Difficulty,
    pub segments: Vec<Segment>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Segment {
    pub spawn: Option<UVec2>,
    pub enemies: Vec<Enemy>,
    pub entities: Vec<Entity>,
    pub tiles: Vec<MapTile>,
}
