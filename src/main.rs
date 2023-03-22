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

use self::scene::*;

mod audio;
mod renderer;
mod scene;
mod task;

mod editor;

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
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Game {
    completed: Vec<SceneId>,
}

impl Game {
    const SAVE: &str = "./assets/save.json";

    pub fn new(scene: &mut Scene) -> Self {
        let file = File::open("./assets/save.json").ok();

        let game = file.map(|file| json::from_reader(file).unwrap());
        game.unwrap_or_default()
    }

    pub fn update(&mut self, scene: &mut Scene) {}

    pub fn on_destroy(&mut self, scene: &mut Scene) {
        let contents = json::to_string_pretty(self).unwrap();

        if write(Self::SAVE, contents).is_err() {
            let msg = "Due to un unexpected error, the game could not be saved and your progress will be lost.";
            let _ = show_simple_message_box(MessageBoxFlag::ERROR, "Saving Game", msg, None);
        }
    }
}
