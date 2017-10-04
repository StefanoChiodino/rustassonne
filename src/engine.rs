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
        let mut tiles = HashMap::new();

        tiles.insert([0, 0].into(), (Tile::new(), Orientation::Up));

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
        let new_engine = Engine {
            board: new_board,
            tile_pool: self.tile_pool.clone(),
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
                    .get_board()
                    .contains_key(&Coordinate::from([0, 0])));
    }

    #[test]
    fn test_new_only_has_single_tile() {
        assert_eq!(1, Engine::new().get_board().len());
    }

    #[test]
    fn test_coord_exists_in_tiles_after_place_next() {
        let engine = Engine::new();
        let coordinate = Coordinate::from([0, 1]);
        let new_engine = engine
            .place_next(coordinate.clone(), Orientation::Up)
            .unwrap();

        assert!(!engine.get_board().contains_key(&coordinate));
        assert!(new_engine.get_board().contains_key(&coordinate));
    }

    #[test]
    fn test_tile_exists_in_tiles_after_place() {
        let engine = Engine::new();
        let coordinate = Coordinate::from([0, 1]);
        let tile = Tile::new();
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
    fn test_cannot_place_on_center() {
        let engine = Engine::new();
        let result = engine.place_next([0, 0], Orientation::Up);

        assert_eq!(result,
                   Err(vec![PlacementError::TileAlreadyAtCoordinate, PlacementError::NotAdjacent]));
    }

    fn test_can_get_next_tile() {
        let engine = Engine::new();

        let next_tile = engine.get_next_tile();

        assert!(next_tile.is_some());
    }
}
