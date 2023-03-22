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
use sdl2::{AudioSubsystem, VideoSubsystem};
use serde::{Deserialize, Serialize};
use serde_json as json;
use task::TaskManager;

use self::scene::*;

mod audio;
mod renderer;
mod scene;
mod task;

mod editor;

pub trait Layer {
    fn new(video: VideoSubsystem, audio: AudioSubsystem) -> Self
    where
        Self: Sized;

    fn update(&mut self);

    /// All window events that make it to an implementation of Layer are
    /// guaranteed to belong to that layers window.
    fn handle_events(&mut self, events: &mut dyn Iterator<Item = &Event>);

    fn should_close(&self) -> bool;

    fn window(&self) -> &Window;
    fn window_mut(&mut self) -> &mut Window;

    fn show(&mut self) {
        self.window_mut().show();
    }

    fn hide(&mut self) {
        self.window_mut().hide();
    }
}

struct Runtime;
impl Layer for Runtime {
    fn new(video: VideoSubsystem, audio: AudioSubsystem) -> Self {
        Self
    }

    fn update(&mut self) {}

    fn handle_events(&mut self, events: &mut dyn Iterator<Item = &Event>) {}

    fn should_close(&self) -> bool {
        false
    }

    fn window(&self) -> &Window {
        todo!()
    }

    fn window_mut(&mut self) -> &mut Window {
        todo!()
    }
}

fn main() {
    let sdl = sdl2::init().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();
    let video = sdl.video().unwrap();
    let audio = sdl.audio().unwrap();

    let mut layers: [Option<Box<dyn Layer>>; 1] = [
        // Some(Box::new(Runtime::new(video.clone(), audio.clone()))),
        Some(Box::new(Editor::new(video.clone(), audio.clone()))),
    ];

    loop {
        let events: Vec<_> = event_pump.poll_iter().collect();

        for layer in &mut layers {
            if let Some(layer) = layer {
                let layer_id = layer.window().id();

                let mut iter = events.iter().filter(|event| match event {
                    Event::Window { window_id, .. } => *window_id == layer_id,
                    Event::Quit { .. } => false,
                    _ => true,
                });

                layer.handle_events(&mut iter);
                layer.update();
            }

            if layer.as_ref().map(|l| l.should_close()).unwrap_or(false) {
                layer.take();
            }
        }

        if layers.iter().all(Option::is_none) {
            break;
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
