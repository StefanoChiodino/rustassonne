//! A Service that can play a game of Carcassonne.

use models::orientation::Orientation;
use models::coordinate::Coordinate;

#[derive(Debug, PartialEq)]
pub enum PlacementError {
    TileAlreadyAtCoordinate,
}

type Result<T> = ::std::result::Result<T, PlacementError>;

#[derive(Debug)]
pub struct Engine {}

impl Engine {
    pub fn new() -> Self {
        Engine {}
    }

    pub fn place_next<T: Into<Coordinate>>(
        &self,
        coordinate: T,
        orientation: Orientation)
            -> Result<()>
    {
        Err(PlacementError::TileAlreadyAtCoordinate)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_engine_cannot_place_on_center_for_first_turn() {
        let engine = Engine::new();
        let result = engine.place_next([0, 0], Orientation::Up);

        assert_eq!(result, Err(PlacementError::TileAlreadyAtCoordinate));
    }
}
