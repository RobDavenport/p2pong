use macroquad::prelude::{draw_circle, vec2, Vec2, WHITE};
use serde::{Deserialize, Serialize};

use crate::paddle::Paddle;
use crate::{blend::Blend, collision_result::CollisionResult, game};
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

const SPEED_INCREASE: f32 = 1.05; // 5% per hit.
const BALL_VELOCITY_X: f32 = 150.0;
const BALL_VELOCITY_Y: f32 = 50.0;

#[derive(Clone, Serialize, Deserialize)]
pub struct Ball {
    pub position: Vec2,
    pub velocity: Vec2,
    pub radius: f32,
}

impl Ball {
    pub fn new() -> Self {
        Self {
            position: vec2(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
            velocity: vec2(BALL_VELOCITY_X, BALL_VELOCITY_Y),
            radius: 5.0,
        }
    }

    // "Tick" our ball. Checks collisions and updates its own
    // position based on its velocity.
    // May reset the ball if it has collided with the left or right side.
    pub fn update(&mut self, paddles: &[Paddle; 2]) -> Option<usize> {
        let mut result = None;

        if let Some(collision) = self.check_collisions(paddles) {
            match collision {
                CollisionResult::Top | CollisionResult::Bottom => {
                    self.velocity.y = -self.velocity.y
                }
                CollisionResult::Left => {
                    *self = Self::new();
                    result = Some(1)
                }
                CollisionResult::Right => {
                    *self = Self::new();
                    self.velocity = -self.velocity;
                    result = Some(0)
                }
                CollisionResult::Paddle(angle) => self.velocity = self.velocity.length() * angle,
            };

            // Get faster each time it collides with something
            // This should really only be called if its Top or Paddle, but isn't game breaking
            // so we can just leave it for code clarity
            self.velocity *= SPEED_INCREASE;
        }

        self.position += self.velocity * game::TICK_TIME;

        result
    }

    // Check all collisions of the ball
    // Includes checks against:
    //   Floor & Roof
    //   Left & Right sides
    //   Passed in paddles
    fn check_collisions(&self, paddles: &[Paddle; 2]) -> Option<CollisionResult> {
        // Collision against top & bottom of field
        if self.position.y - self.radius <= 0.0 {
            return Some(CollisionResult::Top);
        } else if self.position.y + self.radius >= SCREEN_HEIGHT {
            return Some(CollisionResult::Bottom);
        }

        // Collision against left & right of field
        if self.position.x - self.radius <= 0.0 {
            return Some(CollisionResult::Left);
        } else if self.position.x + self.radius >= SCREEN_WIDTH {
            return Some(CollisionResult::Right);
        }

        // Collision against paddles
        for paddle in paddles.iter() {
            if let Some(angle) = paddle.check_collision_against_ball(self) {
                return Some(CollisionResult::Paddle(angle));
            }
        }

        None
    }

    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.radius, WHITE);
    }
}

impl Blend for Ball {
    fn blend(&self, previous: &Self, alpha: f32) -> Self {
        Self {
            position: self.position.blend(&previous.position, alpha),
            velocity: self.velocity.blend(&previous.velocity, alpha),
            radius: self.radius.blend(&previous.radius, alpha),
        }
    }
}
