use sdl2::event::*;
use sdl2::render::*;
use sdl2::video::*;
use sdl2::VideoSubsystem;

const SDL_WINDOW_INPUT_FOCUS: u32 = 0x00000200;
const SDL_WINDOW_MOUSE_FOCUS: u32 = 0x00000400;

use crate::Layer;

pub struct Editor {
    system: VideoSubsystem,
    canvas: WindowCanvas,
}

impl Layer for Editor {
    fn new(system: VideoSubsystem) -> Self {
        let window = system.window("Editor", 800, 600).build().unwrap();
        let canvas = window.into_canvas().accelerated().build().unwrap();
        Self { system, canvas }
    }

    fn update(&mut self) {}

    fn handle_events<'a>(&mut self, events: impl Iterator<Item = &'a Event>) {
        let window = self.canvas.window();

        // We should only handle window events that belongs to this window.
        let events = events.filter(|e| {
            if let Event::Window { window_id, .. } = e && &window.id() == window_id  {
               true
            } else {
                false
            }
        });

        for event in events {
            match event {
                _ => {}
            }

            if window.window_flags() & SDL_WINDOW_INPUT_FOCUS != 0 {
                println!("keyboard focus");
            }

            if window.window_flags() & SDL_WINDOW_MOUSE_FOCUS != 0 {
                println!("mouse focus");
            }
        }
    }

    fn window(&self) -> &Window {
        self.canvas.window()
    }

    fn window_mut(&mut self) -> &mut Window {
        self.canvas.window_mut()
    }
}

// pub enum Pipe {
//     TopLeft,
//     TopRight,
//     Left,
//     Right,
// }

// pub enum Hidden {
//     //  powerups...
// }

// pub enum Block {
//     Sky,
//     Pipe(Pipe),
//     Grass,
//     Hidden(Hidden),
//     Brick,
//     Fake,
// }

// struct Tilemap {
//     block_map: [[Block; map_height]; map_width],
//     current_block: Block,
// }

// impl Tilemap {
//     fn new(&mut self) -> Self {
//         Self {
//             tilemap: [Block::Sky; map_height * map_width],
//         }
//     }

//     pub fn replace_block(&mut self, block_index: Vec2) {
//         self.block_map[block_index.x][block_index.y] = self.current_block;
//     }

//     pub fn get_mouse_index() -> Vec2 {}
// }
