use sdl2::render::*;
use sdl2::video::*;
use sdl2::{AudioSubsystem, VideoSubsystem};

use crate::renderer::*;
use crate::Layer;

pub struct Runtime<'a> {
    video: VideoSubsystem,
    audio: AudioSubsystem,
    canvas: WindowCanvas,

    should_close: bool,

    renderer: Renderer<'a>,
}

impl Layer for Runtime<'_> {
    fn new(video: sdl2::VideoSubsystem, audio: sdl2::AudioSubsystem) -> Self
    where
        Self: Sized,
    {
        todo!()
    }

    fn update(&mut self, keyboard: sdl2::keyboard::KeyboardState, mouse: sdl2::mouse::MouseState) {
        todo!()
    }

    fn handle_events(&mut self, events: &mut dyn Iterator<Item = &sdl2::event::Event>) {
        todo!()
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
