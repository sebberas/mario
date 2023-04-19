use glam::*;
use sdl2::event::*;
use sdl2::keyboard::{KeyboardState, Keycode, Mod};
use sdl2::messagebox::{
    show_message_box, ButtonData, ClickedButton, MessageBoxButtonFlag, MessageBoxColorScheme,
    MessageBoxFlag,
};
use sdl2::mouse::{MouseButton, MouseState, RelativeMouseState};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::*;
use sdl2::video::*;
use sdl2::{AudioSubsystem, VideoSubsystem};

const SDL_WINDOW_INPUT_FOCUS: u32 = 0x00000200;
const SDL_WINDOW_MOUSE_FOCUS: u32 = 0x00000400;

use crate::Layer;

struct EditorState {
    pub tiles: Vec<Option<UVec2>>,
}

pub struct Editor {
    video: VideoSubsystem,
    canvas: WindowCanvas,
    should_close: bool,

    // Command stuff
    mouse_clicks: Vec<MouseButton>,
    key_clicks: Vec<(Keycode, Mod)>,
    saved: bool,
    commands: Vec<Box<dyn Command>>,

    state: EditorState,
}

trait Command {
    fn apply(&mut self, state: &mut EditorState) {}
    fn undo(&mut self, state: &mut EditorState) {}

    fn is_complete(&mut self) -> bool {
        true
    }
}

struct Insert(UVec2);

impl Command for Insert {
    fn apply(&mut self, state: &mut EditorState) {
        if let Some(tile) = state.tiles.iter_mut().find(|tile| tile.is_none()) {
            *tile = Some(self.0)
        } else {
            state.tiles.push(Some(self.0))
        };
    }

    fn undo(&mut self, state: &mut EditorState) {
        if let Some(tile) = state.tiles.iter_mut().find(|tile| tile.contains(&self.0)) {
            *tile = None;
        }
    }
}

struct InsertMany {
    from: UVec2,
    to: Option<UVec2>,
}

impl Command for InsertMany {
    fn apply(&mut self, state: &mut EditorState) {
        let Self { from, to } = self;
        let to = to.unwrap();
    }

    fn is_complete(&mut self) -> bool {
        self.to.is_some()
    }
}

struct Remove(UVec2);

impl Command for Remove {
    fn apply(&mut self, state: &mut EditorState) {
        if let Some(tile) = state.tiles.iter_mut().find(|tile| tile.contains(&self.0)) {
            *tile = None;
        }
    }

    fn undo(&mut self, state: &mut EditorState) {
        if let Some(tile) = state.tiles.iter_mut().find(|tile| tile.is_none()) {
            *tile = Some(self.0)
        } else {
            state.tiles.push(Some(self.0))
        };
    }
}

enum SaveButton {
    Save,
    Discard,
    Cancel,
}

impl Editor {
    const TILES: u32 = 16;

    fn show_save_message_box(&self) -> SaveButton {
        const BUTTONS: [ButtonData; 3] = [
            ButtonData {
                flags: MessageBoxButtonFlag::empty(),
                button_id: 2,
                text: "Cancel",
            },
            ButtonData {
                flags: MessageBoxButtonFlag::empty(),
                button_id: 1,
                text: "Discard",
            },
            ButtonData {
                flags: MessageBoxButtonFlag::empty(),
                button_id: 0,
                text: "Save",
            },
        ];

        const TITLE: &str = "Save Changes";
        const MSG: &str = "This file contains unsaved changes, do you wish to save before exiting?";

        let button = show_message_box(
            MessageBoxFlag::WARNING,
            &BUTTONS,
            TITLE,
            MSG,
            self.window(),
            None,
        );

        match button.unwrap() {
            ClickedButton::CloseButton => SaveButton::Cancel,
            ClickedButton::CustomButton(ButtonData { button_id, .. }) => match button_id {
                0 => SaveButton::Save,
                1 => SaveButton::Discard,
                2 => SaveButton::Cancel,
                _ => unreachable!(),
            },
        }
    }

    fn render(&mut self) {
        let Self { canvas, .. } = self;
        let (width, height) = canvas.window().size();
        let size = height / Self::TILES;

        // Background Color
        canvas.set_draw_color(Color::RGB(229, 231, 235));
        canvas.fill_rect(None).unwrap();

        // Background Grid
        {
            canvas.set_draw_color(Color::RGB(156, 163, 175));
            for i in 0..(width / size) {
                let _ = canvas.draw_line(
                    Point::new((i * size + size) as _, 0),
                    Point::new((i * size + size) as _, height as _),
                );
            }

            for i in 0..Self::TILES {
                let _ = canvas.draw_line(
                    Point::new(0, (i * size + size) as _),
                    Point::new(width as _, (i * size + size) as _),
                );
            }
        }

        self.render_state();
        self.canvas.present();
    }

    fn render_state(&mut self) {
        let Self { canvas, state, .. } = self;
        let (_, height) = canvas.window().size();
        let size = height / Self::TILES;

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        for tile in state.tiles.iter().flatten() {
            let rect = Rect::new((tile.x * size) as _, (tile.y * size) as _, size, size);
            let _ = canvas.fill_rect(rect);
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

            mouse_clicks: Vec::with_capacity(4),
            key_clicks: Vec::with_capacity(16),
            saved: false,
            commands: Vec::with_capacity(64),

            state: EditorState {
                tiles: Vec::with_capacity(16 * 32),
            },
        }
    }

    fn update(&mut self, keyboard: KeyboardState, mouse: MouseState) {
        let Self { canvas, state, .. } = self;
        let (_, height) = canvas.window().size();
        let size = height / Self::TILES;

        let ncommands = self.commands.len();
        for mouse_button in &self.mouse_clicks {
            match mouse_button {
                MouseButton::Left => self.commands.push(Box::new(Insert(uvec2(
                    mouse.x() as u32 / size,
                    mouse.y() as u32 / size,
                )))),
                MouseButton::Right => {
                    let position = uvec2(mouse.x() as u32 / size, mouse.y() as u32 / size);
                    if state.tiles.contains(&Some(position)) {
                        self.commands.push(Box::new(Remove(position)))
                    }
                }
                _ => {}
            }
        }

        for (keycode, keymod) in &self.key_clicks {
            match keycode {
                Keycode::Z if keymod.contains(Mod::LCTRLMOD) => {
                    if let Some(mut command) = self.commands.pop() {
                        command.undo(state);
                    }
                }
                _ => {}
            }
        }

        // We should always apply the latest changes
        if ncommands < self.commands.len() {
            for command in &mut self.commands[ncommands..] {
                if command.is_complete() {
                    command.apply(state)
                }
            }
        }

        self.render();

        self.mouse_clicks.clear();
        self.key_clicks.clear();
    }

    fn handle_events(&mut self, events: &mut dyn Iterator<Item = &Event>) {
        for event in events {
            match event {
                Event::Window { win_event, .. } if *win_event == WindowEvent::Close => {
                    if !self.saved {
                        match self.show_save_message_box() {
                            SaveButton::Save => {
                                // TODO: Save skrrt skrrrt
                                self.should_close = true;
                            }
                            SaveButton::Discard => self.should_close = true,
                            _ => {}
                        }
                    }
                }
                Event::MouseButtonDown { mouse_btn, .. } => self.mouse_clicks.push(*mouse_btn),
                Event::KeyDown {
                    keycode,
                    keymod,
                    repeat,
                    ..
                } if !repeat && keycode.is_some() => {
                    self.key_clicks.push((keycode.unwrap(), *keymod))
                }
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
