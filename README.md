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
