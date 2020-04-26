# ulti-rs

Rust implementation of Ulti, the national card game of Hungary.

This contains:
* A library with the Ulti game logic
* An Ulti game server (soon...)
* An Ulti game client (soon...)

The game logic library will run a single round of Ulti, including bidding,
  playing tricks, and determining the final scores. The game server will wrap
  this in an HTTP service and add round tracking, point tallying, and other
  non-core features. Finally, the client will run in the players' web browsers,
  providing a frontend for the game.
