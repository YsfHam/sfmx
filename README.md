# Simple framework for multimedia projects built with SFML

## Requirements

- SFML rust binding, see <https://docs.rs/sfml/latest/sfml>

## Intoduction

This framework is state machine based.

For a game you could have multiple states:

- Splash screen state: Load some assets for the game
- Game state: handle the in-game events
- Game over state
- Etc.

The main.rs file shows a usage example.
