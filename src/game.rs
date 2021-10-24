use ggrs::{Frame, GGRSRequest, GameInput, GameState, GameStateCell};
use macroquad::prelude::{draw_text, WHITE};
use serde::{Deserialize, Serialize};

use crate::{ball::Ball, blend::Blend, controller::Input, paddle::Paddle, SCREEN_WIDTH};

pub const TICKS_PER_SECOND: f32 = 60.0;
pub const TICK_TIME: f32 = 1.0 / TICKS_PER_SECOND;

const FOURTH_WIDTH: f32 = SCREEN_WIDTH / 4.0;
const SCORE_TOP_OFFSET: f32 = 50.0;
const SCORE_FONT_SIZE: f32 = 50.0;

#[derive(Clone, Serialize, Deserialize)]
pub struct Game {
    game_frame: i32,
    scores: [u8; 2],
    paddles: [Paddle; 2],
    ball: Ball,
}

impl Game {
    pub fn new() -> Self {
        Self {
            game_frame: 0,
            ball: Ball::new(),
            paddles: [Paddle::new(0), Paddle::new(1)],
            scores: [0, 0],
        }
    }

    // Updates everything within the game state.
    pub fn update(&mut self, requests: Vec<GGRSRequest>) {
        for request in requests {
            match request {
                GGRSRequest::SaveGameState { cell, frame } => self.save_game_state(cell, frame),
                GGRSRequest::LoadGameState { cell } => self.load_game_state(cell),
                GGRSRequest::AdvanceFrame { inputs } => self.next_frame(inputs),
            }
        }
    }

    fn save_game_state(&mut self, cell: GameStateCell, frame: Frame) {
        assert_eq!(self.game_frame, frame);
        let buffer = bincode::serialize(&self).unwrap();
        let checksum = fletcher16(&buffer) as u64;

        cell.save(GameState::new(frame, Some(buffer), Some(checksum)));
    }

    fn load_game_state(&mut self, cell: GameStateCell) {
        let state_to_load = cell.load();
        *self = bincode::deserialize(&state_to_load.buffer.unwrap()).unwrap();
    }

    fn next_frame(&mut self, inputs: Vec<GameInput>) {
        inputs.into_iter().enumerate().for_each(|(player, input)| {
            let input: Input = bincode::deserialize(input.input()).unwrap();
            self.paddles[player].input = input.clone();
        });

        self.paddles.iter_mut().for_each(|paddle| {
            paddle.update();
        });

        if let Some(point_scored) = self.ball.update(&self.paddles) {
            self.scores[point_scored] = self.scores[point_scored].wrapping_add(1)
        }

        self.game_frame += 1;
    }

    // Draw everything relevant to the game
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
            SCREEN_WIDTH - FOURTH_WIDTH - SCORE_FONT_SIZE,
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
            game_frame: self.game_frame,
            paddles: [paddle_iter.next().unwrap(), paddle_iter.next().unwrap()],
            ball: self.ball.blend(&previous.ball, alpha),
            scores: self.scores.clone(),
        }
    }
}

/// computes the fletcher16 checksum, copied from wikipedia: <https://en.wikipedia.org/wiki/Fletcher%27s_checksum>
fn fletcher16(data: &[u8]) -> u16 {
    let mut sum1: u16 = 0;
    let mut sum2: u16 = 0;

    for index in 0..data.len() {
        sum1 = (sum1 + data[index] as u16) % 255;
        sum2 = (sum2 + sum1) % 255;
    }

    (sum2 << 8) | sum1
}
