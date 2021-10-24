use macroquad::prelude::{draw_rectangle, vec2, Rect, Vec2, WHITE};

use crate::{ball::Ball, blend::Blend, controller::Input, game, SCREEN_HEIGHT, SCREEN_WIDTH};

const PADDLE_SPEED: f32 = 300.0;

const DEFAULT_WIDTH: f32 = 15.0;
const DEFAULT_HEIGHT: f32 = 75.0;
const EDGE_OFFSET: f32 = 25.0;

#[derive(Clone)]
pub struct Paddle {
    rect: Rect,
    pub input: Option<Input>,
}

impl Paddle {
    pub fn new(player: usize) -> Self {
        let x = if player == 0 {
            EDGE_OFFSET
        } else {
            SCREEN_WIDTH - EDGE_OFFSET - DEFAULT_WIDTH
        };

        Self {
            rect: Rect {
                x,
                y: SCREEN_HEIGHT / 2.0,
                w: DEFAULT_WIDTH,
                h: DEFAULT_HEIGHT,
            },
            input: None,
        }
    }

    // Update the paddles position based on its input
    pub fn update(&mut self) {
        if let Some(input) = &self.input {
            // Move the paddle
            self.rect.y += match input {
                Input::Up => -PADDLE_SPEED * game::TICK_TIME,
                Input::Down => PADDLE_SPEED * game::TICK_TIME,
            };

            // Clamp it from going to high or low off the screen
            self.rect.y = self.rect.y.clamp(0.0, SCREEN_HEIGHT - self.rect.h)
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, WHITE)
    }

    // Checks for a collsion against the ball, and will return
    // the angle of collision
    pub fn check_collision_against_ball(&self, ball: &Ball) -> Option<Vec2> {
        let center = ball.position;
        let halves = vec2(self.rect.w / 2.0, self.rect.h / 2.0);
        let rect_center = vec2(self.rect.x, self.rect.y) + halves;

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
