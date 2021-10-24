use macroquad::prelude::vec2;

use crate::{ball::Ball, blend::Blend, controller::Controller, paddle::Paddle};


const TICKS_PER_SECOND: f32 = 120.0;
pub const TICK_TIME: f32 = 1.0 / TICKS_PER_SECOND;

#[derive(Clone)]
pub struct Game {
    controllers: [Option<Controller>; 2],
    paddles: [Option<Paddle>; 2],
    ball: Option<Ball>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            controllers: [None, None],
            ball: Some(Ball {
                position: vec2(50.0, 50.0),
                velocity: vec2(0.0, 555.0),
                radius: 5.0,
            }),
            paddles: [None, None],
        }
    }

    pub fn read_input(&mut self) {}

    pub fn update(&mut self) {
        self.paddles.iter_mut().for_each(|paddle| {
            if let Some(paddle) = paddle {
                paddle.update();
            }
        });

        if let Some(ball) = &mut self.ball {
            ball.update(&self.paddles);
        }
    }

    pub fn draw(&self) {
        self.paddles.iter().for_each(|paddle| {
            if let Some(paddle) = paddle {
                todo!()
            }
        });

        if let Some(ball) = &self.ball {
            ball.draw();
        }
    }
}

impl Blend for Game {
    fn blend(&self, previous: &Self, alpha: f32) -> Self {
        let ball = match (self.ball.as_ref(), previous.ball.as_ref()) {
            (Some(a), Some(b)) => Some(a.blend(b, alpha)),
            _ => None,
        };

        let paddles_iter = self.paddles.iter().zip(previous.paddles.iter());

        let mut result_iter = paddles_iter.map(|pair| match pair {
            (Some(a), Some(b)) => Some(a.blend(b, alpha)),
            _ => None,
        });

        Self {
            controllers: [None, None],
            paddles: [result_iter.next().unwrap(), result_iter.next().unwrap()],
            ball,
        }
    }
}
