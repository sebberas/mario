use std::fs;
use std::io::BufReader;
use std::path::*;

use ::glam::*;
use ::serde::{Deserialize, Serialize};
use ::serde_json as json;

use crate::map::*;
use crate::scene::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Level {
    pub name: String,
    pub difficulty: Difficulty,
    pub segments: Vec<Segment>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Segment {
    pub spawn: Option<UVec2>,
    pub enemies: Vec<Enemy>,
    pub entities: Vec<Entity>,
    pub tiles: Vec<MapTile>,
    pub background: UVec3,
}

pub fn read_level(path: &impl AsRef<Path>) -> Result<Level, json::Error> {
    let file = fs::File::open(path).unwrap();
    json::from_reader(file)
}

pub fn write_level(path: &impl AsRef<Path>, level: &Level) -> Result<(), json::Error> {
    let file = fs::File::create(path).unwrap();
    json::to_writer_pretty(file, level)
}
