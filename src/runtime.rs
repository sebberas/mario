use glam::*;
use sdl2::event::{Event, WindowEvent};
use sdl2::render::*;
use sdl2::video::*;
use sdl2::{AudioSubsystem, VideoSubsystem};

use crate::renderer::*;
use crate::scene::*;
use crate::Layer;

pub struct Runtime {
    video: VideoSubsystem,
    audio: AudioSubsystem,

    should_close: bool,

    renderer: Renderer,
    scene: Scene,
}

impl Layer for Runtime {
    fn new(video: sdl2::VideoSubsystem, audio: sdl2::AudioSubsystem) -> Self
    where
        Self: Sized,
    {
        let window = video.window("Mario", 1200, 600).build().unwrap();
        let canvas = window.into_canvas().accelerated().build().unwrap();
        let renderer = Renderer::new(canvas);

        let scene = Scene {
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

        Self {
            video,
            audio,
            should_close: false,
            renderer,
            scene,
        }
    }

    fn update(&mut self, keyboard: sdl2::keyboard::KeyboardState, mouse: sdl2::mouse::MouseState) {
        self.renderer.update(&mut self.scene)
    }

    fn handle_events(&mut self, events: &mut dyn Iterator<Item = &sdl2::event::Event>) {
        for event in events {
            match event {
                Event::Window { win_event, .. } if *win_event == WindowEvent::Close => {
                    self.should_close = true;
                }
                _ => {}
            }
        }
    }

    fn should_close(&self) -> bool {
        self.should_close
    }

    fn window(&self) -> &Window {
        self.renderer.canvas.window()
    }

    fn window_mut(&mut self) -> &mut Window {
        self.renderer.canvas.window_mut()
    }
}
