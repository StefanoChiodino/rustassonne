//! A Service that can play a game of Carcassonne.

use std::collections::HashMap;

use models::orientation::Orientation;
use models::coordinate::Coordinate;
use placement_error::PlacementError;
use rules;
use models::tile::Tile;
use standard_tile_factory::StandardTileFactory;
use tile_factory::TileFactory;

type Result<T> = ::std::result::Result<T, Vec<PlacementError>>;

pub type TileMap = HashMap<Coordinate, (Tile, Orientation)>;

#[derive(Debug, Clone, PartialEq)]
pub struct Engine {
    board: TileMap,
    tile_pool: Vec<Tile>,
}

impl Engine {
    pub fn new() -> Self {
        let tile_pool = StandardTileFactory::get_tile_pool();
        Self::new_with_tile_pool(tile_pool)
    }

    pub fn new_with_tile_pool(tile_pool: Vec<Tile>) -> Self {
        let tiles = HashMap::new();

        Engine {
            board: tiles,
            tile_pool: tile_pool,
        }
    }

    // This will avoid tiles ever being mutable.
    pub fn get_board(&self) -> &TileMap {
        &self.board
    }

    pub fn place_next<T: Into<Coordinate>>(&self,
                                           coordinate: T,
                                           orientation: Orientation)
                                           -> Result<Engine> {
        if self.tile_pool.len() >= 1 {
            let next_tile = self.tile_pool[0].clone();
            self.place(next_tile, coordinate, orientation)
        } else {
            Err(vec![PlacementError::PoolOutOfTiles])
        }
    }

    fn place<T: Into<Coordinate>>(&self,
                                  tile: Tile,
                                  coordinate: T,
                                  orientation: Orientation)
                                  -> Result<Engine> {
        let coordinate: Coordinate = coordinate.into();

        let broken_rules = rules::check(self.get_board(), &tile, &coordinate, &orientation);

        if broken_rules.is_err() {
            return Err(broken_rules.unwrap_err());
        }

        let mut new_board = self.board.clone();
        new_board.insert(coordinate, (tile, orientation));
        let (_, new_tile_pool) = self.tile_pool.split_first().unwrap();

        let new_engine = Engine {
            board: new_board,
            tile_pool: new_tile_pool.to_vec(),
        };

        Ok(new_engine)
    }

    pub fn get_next_tile(&self) -> Option<Tile> {
        Some(Tile::new())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_can_place_tile() {
        let tile = Tile::new();
        let engine = Engine::new_with_tile_pool(vec![tile.clone()]);

        let place_result = engine.place(tile, [0, 1], Orientation::Up);

        assert!(place_result.is_ok());
    }

    #[test]
    fn test_placing_not_adjectent_returns_error() {
        let engine = Engine::new();

        let place_result = engine.place_next([0, 99], Orientation::Up);

        assert!(place_result.is_err());
    }

    #[test]
    fn test_tile_exists_after_place_next() {
        let tile = Tile::new();
        let engine = Engine::new_with_tile_pool(vec![tile]);
        let coordinate = Coordinate::from([0, 0]);
        let new_engine = engine
            .place_next(coordinate.clone(), Orientation::Up)
            .unwrap();

        assert!(!engine.get_board().contains_key(&coordinate));
        assert!(new_engine.get_board().contains_key(&coordinate));
    }

    #[test]
    fn test_tile_exists_after_place() {
        let tile = Tile::new();
        let engine = Engine::new_with_tile_pool(vec![tile.clone()]);
        let coordinate = Coordinate::from([0, 0]);
        let orientation = Orientation::Up;

        let new_engine = engine
            .place(tile.clone(), coordinate.clone(), orientation.clone())
            .unwrap();

        let original_contains_tile = engine.get_board().contains_key(&coordinate);
        let new_contained_tile_placement = new_engine.get_board().get(&coordinate).unwrap();

        assert!(!original_contains_tile);
        assert_eq!(new_contained_tile_placement, &(tile, orientation));
    }

    #[test]
    fn test_can_get_next_tile() {
        let tile = Tile::new();
        let engine = Engine::new_with_tile_pool(vec![tile]);

        let next_tile = engine.get_next_tile();

        assert!(next_tile.is_some());
    }
}
