
pub enum Pipe {
    TopLeft,
    TopRight,
    Left,
    Right
}

pub enum Hidden {
    //  powerups...
}

pub enum Block {
    Sky,
    Pipe(Pipe),
    Grass,
    Hidden(Hidden),
    Brick,
    Fake,
}

struct Tilemap {
    block_map: [[Block; map_height]; map_width],
    current_block: Block,
}

impl Tilemap {
    fn new(&mut self) -> Self {
        Self {
            tilemap: [Block::Sky; map_height*map_width] 
        }
    }

    pub fn replace_block(&mut self, block_index: Vec2) {
        self.block_map[block_index.x][block_index.y] = self.current_block;
    }

    pub fn get_mouse_index() -> Vec2 {
        
    }

}