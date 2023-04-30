use std::borrow::Borrow;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use ::glam::*;
use ::sdl2::event::*;
use ::sdl2::keyboard::{KeyboardState, Keycode, Mod};
use ::sdl2::messagebox::*;
use ::sdl2::mouse::{MouseButton, MouseState};
use ::sdl2::pixels::Color;
use ::sdl2::rect::{Point, Rect};
use ::sdl2::render::*;
use ::sdl2::video::*;
use ::sdl2::{AudioSubsystem, VideoSubsystem};
use ::windows::core::ComInterface;
use ::windows::Win32::Foundation::*;
use ::windows::Win32::System::Com::*;
use ::windows::Win32::UI::Shell::*;

use crate::level::read_level;
use crate::os::{windows, MenuItem};

const SDL_WINDOW_INPUT_FOCUS: u32 = 0x00000200;
const SDL_WINDOW_MOUSE_FOCUS: u32 = 0x00000400;

use crate::Layer;

// /// Represents a single command a user has made in the editor.
// ///
// /// A command can consist of multiple steps.
// enum Command {
//     Insert(UVec2),
//     InsertMany { from: UVec2, to: Option<UVec2> },
//     Remove(UVec2),
//     RemoveMany { from: UVec2, to: Option<UVec2> },
// }

struct EditorState {
    pub tiles: Vec<Option<(UVec2, Color)>>,
    pub brush: Rc<RefCell<Color>>,
}

pub struct Editor {
    video: VideoSubsystem,
    canvas: WindowCanvas,
    should_close: Rc<RefCell<bool>>,

    // Command stuff
    mouse_clicks: Vec<MouseButton>,
    key_clicks: Vec<(Keycode, Mod)>,
    saved: bool,

    ncommands: usize,
    commands: Rc<RefCell<Vec<Box<dyn Command>>>>,

    state: EditorState,
}

trait Command: std::fmt::Debug {
    fn apply(&mut self, state: &mut EditorState) {}
    fn undo(&mut self, state: &mut EditorState) {}

    fn is_complete(&mut self) -> bool {
        true
    }
}

#[derive(Debug)]
struct Insert(UVec2, Color);

impl Command for Insert {
    fn apply(&mut self, state: &mut EditorState) {
        if let Some(tile) = state.tiles.iter_mut().find(|tile| tile.is_none()) {
            *tile = Some((self.0, self.1));
        } else {
            state.tiles.push(Some((self.0, self.1)));
        };
    }

    fn undo(&mut self, state: &mut EditorState) {
        if let Some(tile) = state
            .tiles
            .iter_mut()
            .find(|tile| matches!(tile, Some((tile, _)) if tile == &self.0))
        {
            *tile = None;
        }
    }
}

#[derive(Debug)]
struct Remove(UVec2);

impl Command for Remove {
    fn apply(&mut self, state: &mut EditorState) {
        if let Some(tile) = state
            .tiles
            .iter_mut()
            .find(|tile| tile.map_or(false, |_| true))
        {
            *tile = None;
        }
    }

    fn undo(&mut self, state: &mut EditorState) {
        if let Some(tile) = state.tiles.iter_mut().find(|tile| tile.is_none()) {
            *tile = Some((self.0, *RefCell::borrow(&state.brush)))
        } else {
            state
                .tiles
                .push(Some((self.0, *RefCell::borrow(&state.brush))))
        };
    }
}

#[derive(Debug)]
struct Pick {
    new: Color,
    old: Color,
}

impl Command for Pick {
    fn apply(&mut self, state: &mut EditorState) {
        *state.brush.borrow_mut() = self.new;
        println!("yeet");
        println!("{:?}", *RefCell::borrow(&state.brush));
    }

    fn undo(&mut self, state: &mut EditorState) {
        *state.brush.borrow_mut() = self.old;
    }
}

enum SaveButton {
    Save,
    Discard,
    Cancel,
}

impl Editor {
    const TILES: u32 = 16;

    pub fn new(video: VideoSubsystem, audio: AudioSubsystem) -> Self {
        // let window = video.window("Editor", 1600, 800).build().unwrap();
        let mut window = windows::Window::new("Editor");

        let menu = windows::new_menu(vec![
            MenuItem::Action {
                title: "Mario",
                action: Box::new(|| {}),
            },
            MenuItem::Action {
                title: "Luigi",
                action: Box::new(|| {}),
            },
        ]);

        window.set_menu(&menu);

        let mut canvas = window
            .into_sdl2(video.clone())
            .into_canvas()
            .accelerated()
            .build()
            .unwrap();

        canvas.window_mut().show();
        canvas.window_mut().set_size(1600, 800).unwrap();

        Self {
            video,
            canvas,
            should_close: Rc::new(RefCell::new(false)),

            mouse_clicks: Vec::with_capacity(4),
            key_clicks: Vec::with_capacity(16),
            saved: false,

            ncommands: 0,
            commands: Rc::new(RefCell::new(Vec::with_capacity(64))),

            state: EditorState {
                tiles: Vec::with_capacity(16 * 32),
                brush: Rc::new(RefCell::new(Color::RED)),
            },
        }
    }

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

        let button = show_message_box(MessageBoxFlag::WARNING, &BUTTONS, TITLE, MSG, None, None);

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

    fn handle_open_level(&mut self) {
        let dialog: IFileOpenDialog =
            unsafe { CoCreateInstance(&FileOpenDialog, None, CLSCTX_INPROC_SERVER).unwrap() };

        if unsafe { dialog.Show(HWND::default()).is_ok() } {
            let path = unsafe {
                let item = dialog.GetResult().unwrap();
                let path = item.GetDisplayName(SIGDN_FILESYSPATH).unwrap();
                path.to_string().unwrap()
            };

            let level = read_level(&path).unwrap();
            println!("{level:?}");

            self.canvas.window_mut().set_title(&format!(
                "Editor | Name: {} | Segment {}/{} | Path: {path}",
                level.name,
                1,
                level.segments.len()
            ));
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

        // canvas.set_draw_color(Color::RGB(255, 0, 0));
        for (tile, color) in state.tiles.iter().flatten() {
            let rect = Rect::new((tile.x * size) as _, (tile.y * size) as _, size, size);

            canvas.set_draw_color(*color);
            let _ = canvas.fill_rect(rect);
        }
    }
}

impl Layer for Editor {
    fn update(&mut self, keyboard: KeyboardState, mouse: MouseState) {
        let Self { canvas, .. } = self;
        let (_, height) = canvas.window().size();
        let size = height / Self::TILES;

        for mouse_button in &self.mouse_clicks {
            let Self { state, .. } = self;

            match mouse_button {
                MouseButton::Left => {
                    // println!("{:?}", *RefCell::borrow(&state.brush));
                    self.commands.borrow_mut().push(Box::new(Insert(
                        uvec2(mouse.x() as u32 / size, mouse.y() as u32 / size),
                        *RefCell::borrow(&state.brush),
                    )));
                }
                MouseButton::Right => {
                    let position = uvec2(mouse.x() as u32 / size, mouse.y() as u32 / size);
                    if state
                        .tiles
                        .iter()
                        .any(|tile| matches!(tile, Some((tile, ..)) if tile == &position))
                    {
                        self.commands.borrow_mut().push(Box::new(Remove(position)));
                    }
                }
                _ => {}
            }
        }

        for (keycode, keymod) in self.key_clicks.clone() {
            match (keycode, keymod) {
                (Keycode::Z, _) => {
                    if let Some(mut command) = self.commands.borrow_mut().pop() {
                        command.undo(&mut self.state);
                    }
                }
                (Keycode::O, Mod::LCTRLMOD) => {
                    self.handle_open_level();
                }
                (Keycode::S, Mod::LCTRLMOD) => {
                    // Save a level
                }
                (Keycode::AcForward, Mod::LCTRLMOD) => {
                    // Next Segment
                }
                (Keycode::AcBack, Mod::LCTRLMOD) => {
                    // Previous Segment
                }
                _ => {}
            }
        }

        // We should always apply the latest changes
        // println!("{:?}", RefCell::borrow(&self.commands));
        if self.ncommands < RefCell::borrow(&self.commands).len() {
            let Self { state, .. } = self;

            let commands = &mut self.commands.borrow_mut();
            for command in commands[self.ncommands..].iter_mut() {
                if command.is_complete() {
                    command.apply(state)
                }
            }
        }

        self.ncommands = RefCell::borrow(&self.commands).len();
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
                                *self.should_close.borrow_mut() = true;
                            }
                            SaveButton::Discard => *self.should_close.borrow_mut() = true,
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
        *RefCell::borrow(&self.should_close)
    }

    fn window(&self) -> &Window {
        self.canvas.window()
    }

    fn window_mut(&mut self) -> &mut Window {
        self.canvas.window_mut()
    }
}

pub struct EditorTools {
    video: VideoSubsystem,
    audio: AudioSubsystem,

    canvas: WindowCanvas,

    colors: Vec<Color>,

    commands: Rc<RefCell<Vec<Box<dyn Command>>>>,
    should_close: Rc<RefCell<bool>>,
    brush: Rc<RefCell<Color>>,
}

impl EditorTools {
    const WINDOW_WIDTH: u32 = 250;
    const WINDOW_HEIGHT: u32 = 1000;

    const TILES: u32 = 5;
    const TILE_SIZE: u32 = 250 / Self::TILES;

    pub fn new(editor: &Editor, video: VideoSubsystem, audio: AudioSubsystem) -> Self {
        let window = video
            .window("Tile Picker", Self::WINDOW_WIDTH, Self::WINDOW_HEIGHT)
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        Self {
            video,
            audio,
            canvas,

            colors: vec![Color::RED, Color::GREEN, Color::BLUE],

            commands: Rc::clone(&editor.commands),
            should_close: Rc::clone(&editor.should_close),
            brush: Rc::clone(&editor.state.brush),
        }
    }
}

impl Layer for EditorTools {
    fn update(&mut self, keyboard: KeyboardState, mouse: MouseState) {
        let Self { canvas, .. } = self;

        for (i, color) in self.colors.iter().enumerate() {
            canvas.set_draw_color(*color);

            let tile = Rect::new(
                (i % 5 * Self::TILE_SIZE as usize) as _,
                (i / 5 * Self::TILE_SIZE as usize) as _,
                Self::TILE_SIZE as _,
                Self::TILE_SIZE as _,
            );

            canvas.fill_rect(tile).unwrap();
        }

        canvas.present();
    }

    fn handle_events(&mut self, events: &mut dyn Iterator<Item = &Event>) {
        for event in events {
            match event {
                Event::MouseButtonDown {
                    mouse_btn, x, y, ..
                } if mouse_btn == &MouseButton::Left => {
                    let ix = x / (Self::TILE_SIZE as i32);
                    let iy = y / (Self::TILE_SIZE as i32);
                    let i = ix + (iy * Self::TILES as i32);

                    if let Some(brush) = self.colors.get(i as usize) {
                        let old = *RefCell::borrow(&self.brush);
                        if old.ne(brush) {
                            // println!("{brush:?}");

                            self.commands
                                .borrow_mut()
                                .push(Box::new(Pick { new: *brush, old }));
                            println!("{:?}", self.commands.borrow_mut());
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn should_close(&self) -> bool {
        *RefCell::borrow(&self.should_close)
    }

    fn window(&self) -> &Window {
        self.canvas.window()
    }

    fn window_mut(&mut self) -> &mut Window {
        self.canvas.window_mut()
    }
}
