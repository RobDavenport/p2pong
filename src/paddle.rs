use macroquad::prelude::{draw_rectangle, vec2, Vec2, WHITE};
use serde::{Deserialize, Serialize};

use crate::{ball::Ball, blend::Blend, controller::Input, game, SCREEN_HEIGHT, SCREEN_WIDTH};

const PADDLE_SPEED: f32 = 300.0;

const DEFAULT_WIDTH: f32 = 15.0;
const DEFAULT_HEIGHT: f32 = 75.0;
const EDGE_OFFSET: f32 = 25.0;

#[derive(Clone, Serialize, Deserialize)]
pub struct Paddle {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    pub input: Input,
}

impl Paddle {
    pub fn new(player: usize) -> Self {
        let x = if player == 0 {
            EDGE_OFFSET
        } else {
            SCREEN_WIDTH - EDGE_OFFSET - DEFAULT_WIDTH
        };

        Self {
            x,
            y: SCREEN_HEIGHT / 2.0,
            w: DEFAULT_WIDTH,
            h: DEFAULT_HEIGHT,
            input: Input::None,
        }
    }

    // Update the paddles position based on its input
    pub fn update(&mut self) {
        // Move the paddle
        self.y += match self.input {
            Input::Up => -PADDLE_SPEED * game::TICK_TIME,
            Input::Down => PADDLE_SPEED * game::TICK_TIME,
            Input::None => 0.0,
        };

        // Clamp it from going to high or low off the screen
        self.y = self.y.clamp(0.0, SCREEN_HEIGHT - self.h)
    }

    pub fn draw(&self) {
        draw_rectangle(self.x, self.y, self.w, self.h, WHITE)
    }

    // Checks for a collsion against the ball, and will return
    // the angle of collision
    pub fn check_collision_against_ball(&self, ball: &Ball) -> Option<Vec2> {
        let center = ball.position;
        let halves = vec2(self.w / 2.0, self.h / 2.0);
        let rect_center = vec2(self.x, self.y) + halves;

        let distance = center - rect_center;
        let clamped = distance.clamp(-halves, halves);
        let closest = rect_center + clamped;
        let collision_difference = closest - center;

        if collision_difference.length() < ball.radius {
            Some(distance.normalize())
        } else {
            None
        }
    }
}

impl Blend for Paddle {
    fn blend(&self, previous: &Self, alpha: f32) -> Self {
        Self {
            x: self.x.blend(&previous.x, alpha),
            y: self.y.blend(&previous.y, alpha),
            w: self.w.blend(&previous.w, alpha),
            h: self.h.blend(&previous.h, alpha),
            input: self.input.clone(),
        }
    }
}
