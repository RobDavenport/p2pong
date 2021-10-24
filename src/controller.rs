use macroquad::prelude::{is_key_down, KeyCode};

#[derive(Clone)]
pub struct Controller {
    pub input: Option<Input>,
}

#[derive(Clone)]
pub enum Input {
    Up,
    Down,
}

const UP_KEYS: [KeyCode; 2] = [KeyCode::W, KeyCode::Up];
const DOWN_KEYS: [KeyCode; 2] = [KeyCode::S, KeyCode::Down];

impl Controller {
    pub fn new() -> Self {
        Self { input: None }
    }

    pub fn read_input(&mut self, player: usize) {
        if is_key_down(UP_KEYS[player]) {
            self.input = Some(Input::Up)
        } else if is_key_down(DOWN_KEYS[player]) {
            self.input = Some(Input::Down)
        } else {
            self.input = None
        }
    }
}

impl Default for Controller {
    fn default() -> Self {
        Self::new()
    }
}
