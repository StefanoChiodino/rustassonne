//! A Service that can play a game of Carcassonne.

use std::collections::HashSet;

use models::orientation::Orientation;
use models::coordinate::Coordinate;
use placement_error::PlacementError;
use rules;


type Result<T> = ::std::result::Result<T, Vec<PlacementError>>;

#[derive(Debug, Clone, PartialEq)]
pub struct Engine {
    pub tiles: HashSet<Coordinate>,
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
