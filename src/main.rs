#![feature(let_chains)]

use std::fs::*;
use std::path::{Path, PathBuf};
use std::str::SplitInclusive;

use editor::Editor;
use glam::*;
use renderer::Renderer;
use sdl2::event::*;
use sdl2::keyboard::*;
use sdl2::messagebox::*;
use sdl2::video::*;
use sdl2::VideoSubsystem;
use serde::{Deserialize, Serialize};
use serde_json as json;
use task::TaskManager;

use self::map::*;
use self::scene::*;

mod audio;
mod editor;
mod game;
mod input_handler;
mod map;
mod renderer;
mod scene;
mod task;

pub trait Layer: Sized {
    fn new(system: VideoSubsystem) -> Self;

    fn update(&mut self);

    fn handle_events<'a>(&mut self, events: impl Iterator<Item = &'a Event>);

    fn window(&self) -> &Window;
    fn window_mut(&mut self) -> &mut Window;

    fn show(&mut self) {
        self.window_mut().show();
    }

    fn hide(&mut self) {
        self.window_mut().hide();
    }
}

fn main() {
    let sdl = sdl2::init().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();
    let video = sdl.video().unwrap();
    let audio = sdl.audio().unwrap();

    let mut editor = Editor::new(video.clone());

    let window = video.window("Super Mario Bros", 800, 600).build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.default_pixel_format();
    canvas.set_blend_mode(sdl2::render::BlendMode::Blend);

    let mut scene = Scene {
        camera: Camera::new(vec2(0.0, 0.0)),
        enemies: Vec::default(),
        entities: Vec::default(),
        sprites: vec![Sprite::new(
            (uvec2(0, 0), uvec2(16, 16)),
            String::from("assets/sprites/mario_test.png"),
        )],
        text: Vec::default(),
        map_tiles: Vec::default(),
        background: vec4(0.0, 1.0, 1.0, 0.0).into(),
    };

    // let mut renderer = Renderer::new(&mut canvas);

    // let mut game = Game::new(&mut scene);

    'running: loop {
        let events: Vec<_> = event_pump.poll_iter().collect();
        editor.handle_events(events.iter());

        for event in events {
            match event {
                Event::Quit { .. } => break 'running,
                Event::Window {
                    window_id,
                    win_event,
                    ..
                } => {
                    if window_id == canvas.window().id() && win_event == WindowEvent::Close {
                        break 'running;
                    }
                }
                _ => {}
            }
        }
        // game.update(&mut scene);
        // renderer.update(&mut scene);
    }
}
