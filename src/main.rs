#![feature(let_chains)]

use glam::*;
use sdl2::event::*;
use sdl2::keyboard::*;
use sdl2::mouse::*;
use sdl2::video::*;

use self::editor::*;
use self::runtime::*;

//mod animation;
mod audio;
mod editor;
mod game;
mod input_handler;
mod level;
mod map;
mod renderer;
mod runtime;
mod scene;
mod task;

mod os;

pub trait Layer {
    fn update(&mut self, keyboard: KeyboardState, mouse: MouseState);

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

fn main() {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let audio = sdl.audio().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();

    let runtime = Runtime::new(video.clone(), audio.clone());
    let mut layers: Vec<Option<Box<dyn Layer>>> = vec![Some(Box::new(runtime))];

    if std::env::args().any(|arg| arg.contains("editor")) {
        let editor = Box::new(Editor::new(video.clone(), audio.clone()));
        let editor_tools = Box::new(EditorTools::new(&editor, video, audio));
        layers.push(Some(editor));
        layers.push(Some(editor_tools));
    }

    loop {
        let events: Vec<_> = event_pump.poll_iter().collect();

        for layer in &mut layers {
            if let Some(layer) = layer {
                let window_id = layer.window().id();

                let mut iter = events.iter().filter(|event| matches!(event.get_window_id(), Some(id) if id == window_id && !matches!(event, Event::Quit { .. })));

                layer.handle_events(&mut iter);
                layer.update(event_pump.keyboard_state(), event_pump.mouse_state());
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
