use macroquad::prelude::{draw_circle, Vec2, WHITE};

use crate::paddle::Paddle;
use crate::SCREEN_HEIGHT;
use crate::{blend::Blend, collision_result::CollisionResult, game::TICK_RATE};

#[derive(Clone)]
pub struct Ball {
    pub position: Vec2,
    pub velocity: Vec2,
    pub radius: f32,
}

impl Ball {
    fn check_collisions(&self, paddles: &[Option<Paddle>; 2]) -> Option<CollisionResult> {
        // Collision against top & bottom of field
        if self.position.y - self.radius <= 0.0 {
            println!("collision top");
            return Some(CollisionResult::CollisionTop);
        } else if self.position.y + self.radius >= SCREEN_HEIGHT {
            println!("collision bottom");
            return Some(CollisionResult::CollisionBottom);
        }

        // Collision against paddles
        paddles.iter().for_each(|paddle| {
            if let Some(paddle) = paddle {
                todo!();
            }
        });

        None
    }

    pub fn update(&mut self, paddles: &[Option<Paddle>; 2]) {
        if let Some(collision) = self.check_collisions(paddles) {
            match collision {
                CollisionResult::CollisionTop | CollisionResult::CollisionBottom => {
                    self.velocity.y = -self.velocity.y
                }
                CollisionResult::CollisionScore => todo!(),
                CollisionResult::CollisionPaddle => todo!(),
            }
        }

        self.position += self.velocity * TICK_RATE
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
