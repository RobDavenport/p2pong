use macroquad::prelude::Rect;

use crate::{blend::Blend, controller::Input, game::TICK_RATE};

const PADDLE_SPEED: f32 = 5.0;

#[derive(Clone)]
pub struct Paddle {
    rect: Rect,
    input: Option<Input>,
}

impl Paddle {
    pub fn update(&mut self) {
        if let Some(input) = &self.input {
            self.rect.y += match input {
                Input::Up => -PADDLE_SPEED * TICK_RATE,
                Input::Down => PADDLE_SPEED * TICK_RATE,
            }
        }
    }
}

impl Blend for Paddle {
    fn blend(&self, previous: &Self, alpha: f32) -> Self {
        Self {
            rect: Rect {
                x: self.rect.x.blend(&previous.rect.x, alpha),
                y: self.rect.y.blend(&previous.rect.y, alpha),
                w: self.rect.w.blend(&previous.rect.w, alpha),
                h: self.rect.h.blend(&previous.rect.h, alpha),
            },
            input: self.input.clone(),
        }
    }
}
