//! A Service that can play a game of Carcassonne.

use std::collections::HashSet;

use models::orientation::Orientation;
use models::coordinate::Coordinate;
use placement_error::PlacementError;
use rules;
use models::tile::Tile;


type Result<T> = ::std::result::Result<T, Vec<PlacementError>>;

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

    // This will avoid tiles being ever mutable.
    pub fn get_tiles(&self) -> &HashSet<Coordinate> {
        &self.tiles
    }

    pub fn place_next<T: Into<Coordinate>>(&self,
                                           coordinate: T,
                                           orientation: Orientation)
                                           -> Result<Engine> {
        let next_tile = Tile::new();

        self.place(next_tile, coordinate, orientation)
    }

    fn place<T: Into<Coordinate>>(&self,
                                  tile: Tile,
                                  coordinate: T,
                                  orientation: Orientation)
                                  -> Result<Engine> {
        let coordinate: Coordinate = coordinate.into();

        let broken_rules = rules::check(&self, &coordinate);
        if broken_rules.is_err() {
            return Err(broken_rules.unwrap_err());
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
    fn test_can_place_tile() {
        let engine = Engine::new();
        let tile = Tile::new();

        let new_engine = engine.place(tile, [0, 1], Orientation::Up);

        assert!(new_engine.is_ok());
    }

    #[test]
    fn test_can_place_next_tile() {
        let engine = Engine::new();

        let new_engine = engine.place_next([0, 1], Orientation::Up);

        assert!(new_engine.is_ok());
    }

    #[test]
    fn test_placing_not_adjectent_returns_error() {
        let engine = Engine::new();

        let new_engine = engine.place_next([0, 99], Orientation::Up);

        assert!(new_engine.is_err());
    }

    #[test]
    fn test_new_has_center_tile() {
        assert!(Engine::new()
                    .get_tiles()
                    .contains(&Coordinate::from([0, 0])));
    }

    #[test]
    fn test_new_only_has_single_tile() {
        assert_eq!(1, Engine::new().get_tiles().len());
    }

    #[test]
    fn test_coord_exists_in_tiles_after_place_next() {
        let engine = Engine::new();
        let coordinate = Coordinate::from([0, 1]);
        let new_engine = engine
            .place_next(coordinate.clone(), Orientation::Up)
            .unwrap();

        assert!(!engine.get_tiles().contains(&coordinate));
        assert!(new_engine.get_tiles().contains(&coordinate));
    }
}