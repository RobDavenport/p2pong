use macroquad::prelude::{is_key_down, KeyCode};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum Input {
    None,
    Up,
    Down,
}

// Convert local inputs into our "Input" struct
pub fn local_input() -> Input {
    if is_key_down(KeyCode::W) {
        Input::Up
    } else if is_key_down(KeyCode::S) {
        Input::Down
    } else {
        Input::None
    }
}
