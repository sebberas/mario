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

// impl Sprite {
//     pub fn from_block(block: &Block) -> Sprite {
//         // alt nedenstående skal hardcodes ind :'(

//         match block {
//             Block::Ground => Sprite::new((uvec2(0, 10), uvec2(0, 10)),
// String::from("ground")),             Block::Question(..) => {
//                 Sprite::new((uvec2(0, 10), uvec2(0, 10)),
// String::from("question"))             }
//             _ => unreachable!(),
//         }
//     }
// }

impl ToSprite for Block {
    fn to_sprite(&self) -> Sprite {
        match self {
            Block::Ground => Sprite::new(
                (uvec2(0, 16), uvec2(16, 32)),
                String::from("./sprites/tilesheet.png"),
            ),
            Block::Question { .. } => todo!(),
        }
    }
}
