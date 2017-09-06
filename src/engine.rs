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
        let coord: Coordinate = coordinate.into();

        if coord == [0, 0].into() {
            Err(PlacementError::TileAlreadyAtCoordinate)
        } else {
            Ok(())
        }
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

        assert_eq!(result, Ok(()));
    }
}
