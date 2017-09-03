# Rustassonne - Carcassonne in Rust

We'll be following these rules to implement the board game Carcassonne in Rust.

## Entities

- Tile
  - Features: Feature []
- Road imp Feature
  - Connection: Directions[]
- City imp Feature
  - Connection: Directions[]
  - Badge: bool
- Monastery imp Feature
- Directions: Enum[]
  - North
  - East
  - South
  - West
- Coordinate: find crate!
- GameState:
  - NumberOfPlayers: u32
  - CurrentTurn: u32
  - Cells: Map<Coordinate, Tile>
  - BoardId: guid

## Feature needed

### Tile cheker

Makes sure that a tile is valid. E.g. a road has 2,3, or 4 sides.

### Point resolution:

E.g. when placing road down if the road is closed then we need to get the points and initiate a resolution

## Draft lower layer Api calls

- Start Game (number of players)
- Get Next Tile () -> Tile
- Get Next Turn Player Id () -> i32
- Play Turn (TileCoordinate, AddFollowerSides [] ) -> AddTileResult
- Get Board -> Board

## Architecture

At the moment the game will require 3 conceptual layers:

- The core concept of the game. E.g. what is a tile? When does the game stop?
- A game layer that leverages the core one and offers API to start a game, play turns, etc.
- UI layer. This could be cosole or web for example.
