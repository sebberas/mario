use glam::*;
use sdl2::pixels::*;
use serde::{Deserialize, Serialize};

use crate::renderer::*;
use crate::scene::Sprite;

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum Block {
    Ground,
    Question,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct MapTile {
    pub block: Block,
    pub coordinate: UVec2,
}

impl Sprite {
    pub fn from_block(block: &Block) -> Sprite {
        // alt nedenstående skal hardcodes ind :'(

        match block {
            Block::Ground => Sprite::new((uvec2(0, 10), uvec2(0, 10)), String::from("ground")),
            Block::Question => Sprite::new((uvec2(0, 10), uvec2(0, 10)), String::from("question")),
            _ => unreachable!(),
        }
    }
}
