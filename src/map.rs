use glam::*;
use sdl2::pixels::*;
use serde::{Deserialize, Serialize};

use crate::renderer::*;
use crate::scene::Sprite;

#[derive(Debug, Deserialize, Serialize)]
pub enum Block {
    Ground { asset_path: String },
    Question { asset_path: String },
}

impl Block {
    pub fn asset(&self) -> String {
        match self {
            Self::Ground { asset_path } => String::from("assets/sprites/mario_test.png"),
            Self::Question { asset_path } => String::from("assets/sprites/mario_test.png"),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MapTile {
    pub block: Block,
    pub coordinate: UVec2,
}
