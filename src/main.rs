mod ball;
mod blend;
mod collision_result;
mod controller;
mod game;
mod paddle;

use std::{
    net::SocketAddr,
    time::{Duration, Instant},
};

use blend::Blend;
use controller::Input;
use game::*;
use ggrs::{GGRSError, P2PSession, PlayerType, SessionState};
use macroquad::prelude::*;

const SCREEN_HEIGHT: f32 = 600.0;
const SCREEN_WIDTH: f32 = 800.0;
const FRAME_AHEAD_SLOWDOWN_AMOUNT: f32 = 1.1;

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

const P1_PORT: u16 = 7000;
const P2_PORT: u16 = 7001;
const LOCAL_IP: &str = "127.0.0.1";

#[macroquad::main(window_conf)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the environment variables
    let args: Vec<String> = std::env::args().collect();
    let player_arg = args.get(1).expect("Please enter p1 or p2");

    let input_size = bincode::serialized_size(&Input::None).unwrap() as usize;

    // Set up the session for
    let (mut sess, local_handle) = if "p1" == player_arg {
        let mut sess = P2PSession::new(2, input_size, P1_PORT)?;
        sess.add_player(PlayerType::Local, 0)?;
        sess.add_player(PlayerType::Remote(get_remote_addr(0, &args)), 1)?;
        (sess, 0)
    } else if "p2" == player_arg {
        let mut sess = P2PSession::new(2, input_size, P2_PORT)?;
        sess.add_player(PlayerType::Local, 1)?;
        sess.add_player(PlayerType::Remote(get_remote_addr(1, &args)), 0)?;
        (sess, 1)
    } else {
        panic!("Invalid player argument, please use p1 or p2")
    };

    sess.set_fps(game::TICKS_PER_SECOND as u32)?;
    sess.start_session()?;

    // Set up some initial state
    let mut previous_state = None;
    let mut current_state = Game::new();

    let mut current_time = Instant::now();
    let mut accumulator = Duration::new(0, 0);
    let tick_time = Duration::from_secs_f32(game::TICK_TIME);
    let slow_tick_time = Duration::from_secs_f32(game::TICK_TIME * FRAME_AHEAD_SLOWDOWN_AMOUNT);

    loop {
        // Poll remote clients for inputs
        sess.poll_remote_clients();

        for event in sess.events() {
            println!("GGRS Event: {:?}", event);
        }

        // We only want to update the game state if our session
        // is properly running
        if sess.current_state() == SessionState::Running {
            // Get the current time
            let new_time = Instant::now();
            let frame_time = new_time - current_time;
            current_time = new_time;

            // Track our progress until the next update tick.
            accumulator += frame_time;

            // Run the simulation a bit slower to let other
            // clients catch up
            let this_tick_time = if sess.frames_ahead() > 0 {
                slow_tick_time
            } else {
                tick_time
            };

            // Since we separate the rendering from the update logic
            // We might update multiple times per frame, or even zero
            while accumulator >= this_tick_time {
                accumulator -= this_tick_time;

                let local_input = controller::local_input();
                let local_input = bincode::serialize(&local_input).unwrap();
                previous_state = Some(current_state.clone());

                match sess.advance_frame(local_handle, &local_input) {
                    Ok(requests) => current_state.update(requests),
                    Err(GGRSError::PredictionThreshold) => println!("Frame Skipped"),
                    Err(e) => return Err(Box::new(e)),
                };
            }
        }

        // Begin the drawing
        clear_background(BLACK);

        // In the case where we are rendering a frame between updates, we
        // have to interpolate the game state to create a smooth image,
        // otherwise we may have a choppy image if the game updates
        // slower than our FPS.
        if previous_state.is_some() {
            let alpha = accumulator.as_secs_f32() / tick_time.as_secs_f32();
            let blended = Game::blend(&current_state, previous_state.as_ref().unwrap(), alpha);
            blended.draw()
        } else {
            current_state.draw()
        };

        next_frame().await
    }
}

fn get_remote_addr(player_id: usize, args: &[String]) -> SocketAddr {
    let ip = if let Some(ip) = args.get(2) {
        ip.clone()
    } else {
        LOCAL_IP.to_string()
    };

    format!("{}:{}", ip, if player_id == 0 { P2_PORT } else { P1_PORT })
        .parse()
        .unwrap()
}
