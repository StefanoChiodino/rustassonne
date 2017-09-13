//! A Service that can play a game of Carcassonne.

use std::collections::HashSet;

use models::orientation::Orientation;
use models::coordinate::Coordinate;

#[derive(Debug, PartialEq)]
pub enum PlacementError {
    NotAdjacent,
    TileAlreadyAtCoordinate,
}

type Result<T> = ::std::result::Result<T, PlacementError>;

#[derive(Debug, Clone, PartialEq)]
pub struct Engine {
    tiles: HashSet<Coordinate>,
}

impl Engine {
    pub fn new() -> Self {
        let mut tiles: HashSet<Coordinate> = HashSet::new();

        tiles.insert([0, 0].into());

        Engine { tiles: tiles }
    }

    pub fn place_next<T: Into<Coordinate>>(&self,
                                           coordinate: T,
                                           orientation: Orientation)
                                           -> Result<Engine> {
        let coordinate: Coordinate = coordinate.into();

        if self.tiles.contains(&coordinate) {
            return Err(PlacementError::TileAlreadyAtCoordinate);
        }

        //////// UNDERENGINEERED
        //////// TODO: OVERENGINEER
        let hasAdjecentTiles = self.tiles
            .contains(&[coordinate.x, coordinate.y - 1].into()) ||
                               self.tiles
                                   .contains(&[coordinate.x, coordinate.y + 1].into()) ||
                               self.tiles
                                   .contains(&[coordinate.x + 1, coordinate.y].into()) ||
                               self.tiles
                                   .contains(&[coordinate.x - 1, coordinate.y].into());

        if !hasAdjecentTiles {
            return Err(PlacementError::NotAdjacent);
        }

        let mut new_tiles = self.tiles.clone();
        new_tiles.insert(coordinate);
        let new_engine = Engine { tiles: new_tiles };

        Ok(new_engine)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_engine_cannot_place_on_center() {
        let engine = Engine::new();
        let result = engine.place_next([0, 0], Orientation::Up);

        assert_eq!(result, Err(PlacementError::TileAlreadyAtCoordinate));
    }

    #[test]
    fn test_engine_can_place_next_to_center() {
        let engine = Engine::new();
        let result = engine.place_next([0, 1], Orientation::Up);

        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_engine_cannot_place_with_a_gap() {
        let engine = Engine::new();

        let result = engine.place_next([0, 2], Orientation::Up);

        assert_eq!(result, Err(PlacementError::NotAdjacent));
    }

    #[test]
    fn test_engine_cannot_place_tiles_in_same_location() {
        let mut engine = Engine::new();
        engine = engine.place_next([0, 1], Orientation::Up).unwrap();

        let result = engine.place_next([0, 1], Orientation::Up);

        assert_eq!(result, Err(PlacementError::TileAlreadyAtCoordinate));
    }
}
