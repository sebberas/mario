use std::time::Duration;

use ::glam::*;
use ::sdl2::event::{Event, WindowEvent};
use ::sdl2::keyboard::*;
use ::sdl2::mouse::*;
use ::sdl2::video::*;
use ::sdl2::{AudioSubsystem, VideoSubsystem};

use crate::audio::AudioManager;
use crate::game::*;
use crate::renderer::*;
use crate::scene::*;
use crate::Layer;

pub struct Runtime {
    video: VideoSubsystem,
    audio: AudioSubsystem,

    should_close: bool,

    renderer: Renderer,
    scene: Scene,

    systems: GameSystems,
    game: Game,
}

impl Runtime {
    pub fn new(video: sdl2::VideoSubsystem, audio: sdl2::AudioSubsystem) -> Self {
        let window = video
            .window("Mario", 1200, 600)
            .resizable()
            .build()
            .unwrap();

        let mut canvas = window
            .into_canvas()
            .accelerated()
            .present_vsync()
            .build()
            .unwrap();

        let renderer = Renderer::new(canvas);

        let mut scene = Scene {
            camera: Camera::new(vec2(0.0, 0.0)),
            enemies: Vec::default(),
            entities: Vec::default(),
            player: Player {
                position: vec2(10.0, 10.0),
                move_velocity: 0.0,
                jump_velocity: 0.0,
                can_jump: true,
                is_shown: true,
            },
            sprites: Vec::default(),
            text: Vec::default(),
            tiles: Vec::default(),
            background: uvec3(146, 144, 255),
        };

        let mut systems = {
            let mut audio_manager = AudioManager::new(audio.clone());
            audio_manager.register_dir(&"./assets/audio", true);

            GameSystems {
                audio: audio_manager,
            }
        };

        let game = Game::new(&mut scene, &systems);

        Self {
            video,
            audio,
            should_close: false,
            renderer,
            scene,

            systems,
            game,
        }
    }
}

impl Layer for Runtime {
    fn update(&mut self, keyboard: KeyboardState, mouse: MouseState) {
        let Self { game, systems, .. } = self;

        game.update(&mut self.scene, systems, keyboard);

        systems.audio.update();

        let start = std::time::Instant::now();
        self.renderer.update(&mut self.scene);
        let end = std::time::Instant::now();

        let elapsed = end - start;
        // println!("{elapsed:?}");
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
