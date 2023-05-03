use ::glam::*;
use ::serde::{Deserialize, Serialize};

use crate::game::BoundingBox;
use crate::renderer::Renderer;
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

impl MapTile {
    pub fn collider(&self) -> BoundingBox {
        let coordinate = self.coordinate.as_vec2();
        let [x, y] = coordinate.as_ref();
        BoundingBox {
            x: *x,
            y: *y,
            width: 16.0,
            height: 16.0,
        }
    }
}

impl ToSprite for Block {
    fn to_sprite(&self) -> Sprite {
        match self {
            Block::Ground => Sprite::new(
                (uvec2(0, 16), uvec2(16, 16)),
                "./assets/sprites/tilesheet.png",
                false,
            ),
            Block::Question { .. } => todo!(),
        }
    }
}
