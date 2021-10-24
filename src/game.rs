use macroquad::prelude::{draw_text, WHITE};

use crate::{ball::Ball, blend::Blend, controller::Controller, paddle::Paddle, SCREEN_WIDTH};

const TICKS_PER_SECOND: f32 = 120.0;
pub const TICK_TIME: f32 = 1.0 / TICKS_PER_SECOND;

const FOURTH_WIDTH: f32 = SCREEN_WIDTH / 4.0;
const SCORE_TOP_OFFSET: f32 = 50.0;
const SCORE_FONT_SIZE: f32 = 50.0;

#[derive(Clone)]
pub struct Game {
    controllers: [Controller; 2],
    scores: [u8; 2],
    paddles: [Paddle; 2],
    ball: Ball,
}

impl Game {
    pub fn new() -> Self {
        Self {
            controllers: [Controller::new(), Controller::new()],
            ball: Ball::new(),
            paddles: [Paddle::new(0), Paddle::new(1)],
            scores: [0, 0],
        }
    }

    pub fn read_input(&mut self) {
        self.controllers
            .iter_mut()
            .enumerate()
            .for_each(|(index, controller)| controller.read_input(index));
    }

    fn handle_input(&mut self) {
        self.controllers
            .iter()
            .zip(self.paddles.iter_mut())
            .for_each(|(controller, paddle)| paddle.input = controller.input.clone());
    }

    pub fn update(&mut self) {
        self.read_input();

        self.handle_input();

        self.paddles.iter_mut().for_each(|paddle| {
            paddle.update();
        });

        if let Some(point_scored) = self.ball.update(&self.paddles) {
            self.scores[point_scored] = self.scores[point_scored].wrapping_add(1)
        }
    }

    pub fn draw(&self) {
        self.paddles.iter().for_each(|paddle| paddle.draw());

        self.ball.draw();

        draw_text(
            &self.scores[0].to_string(),
            FOURTH_WIDTH,
            SCORE_TOP_OFFSET,
            SCORE_FONT_SIZE,
            WHITE,
        );
        draw_text(
            &self.scores[1].to_string(),
            SCREEN_WIDTH - FOURTH_WIDTH,
            SCORE_TOP_OFFSET,
            SCORE_FONT_SIZE,
            WHITE,
        );
    }
}

impl Blend for Game {
    fn blend(&self, previous: &Self, alpha: f32) -> Self {
        let mut paddle_iter =
            self.paddles.iter().zip(previous.paddles.iter()).map(
                |(current_paddle, previous_paddle)| current_paddle.blend(previous_paddle, alpha),
            );

        Self {
            controllers: self.controllers.clone(),
            paddles: [paddle_iter.next().unwrap(), paddle_iter.next().unwrap()],
            ball: self.ball.blend(&previous.ball, alpha),
            scores: self.scores.clone(),
        }
    }
}
