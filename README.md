# p2pong

An implementation of the Atari classic Pong game in Rust.

Uses the macroquad game library to handle window, input, and graphics, and GGRS for the p2p network impementation.

This project is intended to serve as an example of how to build a p2p multiplayer game with GGRS in Rust.

Controls:
```
W -> Move your paddle Up.
S -> Move your paddlw Down.
```

To run the game locally:
1. Open two consoles and enter the following commands:
1. `cargo run p1`
1. `cargo run p2`
1. The game should start after the clients have connected.

This will connect to 127.0.0.1 using port 7000 (for p1) and port 7001 (for p2)

To run remotely:
1. `cargo run [p1/p2] [IP_ADDRESS]`