# p2pong

An implementation of the Atari classic Pong game in Rust.

Uses the macroquad game library to handle window, input, and graphics, and GGRS for the p2p network impementation.

This project is intended to serve as an example of how to build a p2p multiplayer game with GGRS in Rust.

Ports 7000 and 7001 will be used for player 1 and player 2 respectively.

Controls:
```
W -> Move your paddle Up.
S -> Move your paddlw Down.
```

To run the game locally (using 127.0.0.1):
1. Open two consoles and enter the following commands:
1. `cargo run p1`
1. `cargo run p2`
1. The game should start after the clients have connected.

To run remotely given a known peer:
1. `cargo run [p1/p2] [IP_ADDRESS_OF_PEER]`
