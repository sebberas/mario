use glam::*;
use sdl2::pixels::*;
use serde::{Deserialize, Serialize};

use crate::renderer::*;
use crate::scene::*;

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
pub enum Block {
    Ground,
    Question(Option<Item>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct MapTile {
    pub block: Block,
    pub coordinate: UVec2,
}

impl ToSprite for Block {
    fn to_sprite(&self) -> Sprite {
        match self {
            Block::Ground => Sprite::new(
                (uvec2(0, 16), uvec2(16, 16)),
                "./assets/sprites/tilesheet.png",
            ),
            Block::Question { .. } => todo!(),
        }
    }
}
