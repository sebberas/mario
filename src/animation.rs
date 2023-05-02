use ::glam::*;

use crate::scene::*;

pub enum Transition {
    Collect(Item),
}

pub enum State {
    Mini,
    Super,
    Fire,
}

pub enum Action {
    Standing,
    Running { frame: usize },
    Jumping { frame: usize },
}

pub struct Player {
    state: State,
    grounded: bool,
    action: Action,
}

impl ToSprite for Player {
    fn to_sprite(&self) -> Sprite {
        let sprites: &'static [()] = match self.state {
            State::Mini => &[()],
            State::Super => &[(), ()],
            State::Fire => &[(), (), ()],
        };

        todo!()
    }
}
