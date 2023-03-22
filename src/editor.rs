use sdl2::event::*;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::*;
use sdl2::video::*;
use sdl2::{AudioSubsystem, VideoSubsystem};

const SDL_WINDOW_INPUT_FOCUS: u32 = 0x00000200;
const SDL_WINDOW_MOUSE_FOCUS: u32 = 0x00000400;

use crate::Layer;

pub struct Editor {
    video: VideoSubsystem,
    canvas: WindowCanvas,
    should_close: bool,
}

impl Layer for Editor {
    fn new(video: VideoSubsystem, audio: AudioSubsystem) -> Self {
        let window = video.window("Editor", 1600, 1200).build().unwrap();
        let canvas = window.into_canvas().accelerated().build().unwrap();

        Self {
            video,
            canvas,
            should_close: false,
        }
    }

    fn update(&mut self) {
        const TILES: u32 = 16;

        let Self { canvas, .. } = self;

        canvas.set_draw_color(Color::RGB(229, 231, 235));
        canvas.fill_rect(None).unwrap();

        let (width, height) = canvas.window().size();
        let size = height / TILES;

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        for i in 0..(width / size) {
            canvas.draw_line(
                Point::new((i * size + size) as _, 0),
                Point::new((i * size + size) as _, height as _),
            );
        }

        for i in 0..TILES {
            canvas.draw_line(
                Point::new(0, (i * size + size) as _),
                Point::new(width as _, (i * size + size) as _),
            );
        }

        canvas.present();
    }

    fn handle_events(&mut self, events: &mut dyn Iterator<Item = &Event>) {
        for event in events {
            match event {
                Event::Window { win_event, .. } => match *win_event {
                    WindowEvent::Close => self.should_close = true,
                    _ => {}
                },
                _ => {}
            }
        }
    }

    fn should_close(&self) -> bool {
        self.should_close
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
