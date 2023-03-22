use glam::*;
use sdl2::event::*;
use sdl2::mouse::MouseButton;
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
    tiles: Vec<Option<UVec2>>,
}

impl Editor {
    const TILES: u32 = 16;

    fn handle_click(&mut self, btn: MouseButton, position: UVec2) {
        let Self { canvas, tiles, .. } = self;
        let (_, window_height) = canvas.window().size();

        let tile_size = window_height / Self::TILES;
        let tile = uvec2(position.x / tile_size, position.y / tile_size);

        match btn {
            MouseButton::Left => {
                // We can find the tile coordinates by dividing the clicked position by the size
                // of a tile.
                tiles.push(Some(tile));
            }
            MouseButton::Right => {
                let item = tiles.iter_mut().find(|e| e.contains(&tile));
                if let Some(item) = item {
                    let _ = item.take();
                }
            }
            _ => {}
        }
    }
}

impl Layer for Editor {
    fn new(video: VideoSubsystem, audio: AudioSubsystem) -> Self {
        let window = video.window("Editor", 1600, 800).build().unwrap();
        let canvas = window.into_canvas().accelerated().build().unwrap();

        Self {
            video,
            canvas,
            should_close: false,
            tiles: Vec::with_capacity(16 * 32),
        }
    }

    fn update(&mut self) {
        let Self { canvas, .. } = self;

        canvas.set_draw_color(Color::RGB(229, 231, 235));
        canvas.fill_rect(None).unwrap();

        let (width, height) = canvas.window().size();
        let size = height / Self::TILES;

        // Background
        {
            canvas.set_draw_color(Color::RGB(156, 163, 175));
            for i in 0..(width / size) {
                canvas.draw_line(
                    Point::new((i * size + size) as _, 0),
                    Point::new((i * size + size) as _, height as _),
                );
            }

            for i in 0..Self::TILES {
                canvas.draw_line(
                    Point::new(0, (i * size + size) as _),
                    Point::new(width as _, (i * size + size) as _),
                );
            }
        }

        // Tiles
        {
            canvas.set_draw_color(Color::RGB(255, 0, 0));
            for tile in &self.tiles {
                if let Some(tile) = tile {
                    let rect = Rect::new((tile.x * size) as _, (tile.y * size) as _, size, size);
                    canvas.fill_rect(rect);
                }
            }
        }

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.fill_rect(Rect::new((width / 2 - 2 * size) as _, 0, 4 * size, 64));

        canvas.present();
    }

    fn handle_events(&mut self, events: &mut dyn Iterator<Item = &Event>) {
        for event in events {
            match event {
                Event::Window { win_event, .. } => match *win_event {
                    WindowEvent::Close => self.should_close = true,
                    _ => {}
                },
                Event::MouseButtonDown {
                    mouse_btn, x, y, ..
                } => self.handle_click(*mouse_btn, uvec2(*x as _, *y as _)),
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
