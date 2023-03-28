use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct InputHandler {
    mouse_state: sdl2::mouse::MouseState,
}

impl InputHandler {
    pub fn new(&mut self, mouse_state: sdl2::mouse::MouseState) -> InputHandler {
        InputHandler { mouse_state }
    }

    pub fn is_mouse_clicked(&self) -> bool {
        todo!()
    }

    pub fn is_mouse_held(&self) -> bool {
        if self.is_mouse_clicked() {
            return false;
        }

        true
    }
}
