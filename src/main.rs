mod ball;
mod blend;
mod collision_result;
mod controller;
mod game;
mod paddle;

use std::time::{Duration, Instant};

use blend::Blend;
use game::*;
use macroquad::prelude::*;

const SCREEN_HEIGHT: f32 = 400.0;
const SCREEN_WIDTH: f32 = 800.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "P2Pong".to_owned(),
        fullscreen: false,
        window_resizable: false,
        window_height: SCREEN_HEIGHT as i32,
        window_width: SCREEN_WIDTH as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut previous_state = None;
    let mut current_state = Game::new();

    let mut current_time = Instant::now();
    let mut accumulator = Duration::new(0, 0);
    let tick_time = Duration::from_secs_f32(game::TICK_TIME);

    loop {
        let new_time = Instant::now();
        let frame_time = new_time - current_time;
        current_time = new_time;

        accumulator += frame_time;

        while accumulator >= tick_time {
            previous_state = Some(current_state.clone());
            current_state.update();
            accumulator -= tick_time;
        }

        clear_background(BLACK);

        let alpha = accumulator.as_secs_f32() / tick_time.as_secs_f32();
        if previous_state.is_some() {
            let blended = Game::blend(&current_state, previous_state.as_ref().unwrap(), alpha);
            blended.draw();
        } else {
            current_state.draw()
        };

        next_frame().await
    }
}
