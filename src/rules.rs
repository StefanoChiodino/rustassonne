use std::collections::HashMap;

use models::coordinate::Coordinate;
use models::orientation::Orientation;
use models::tile::Tile;
use placement_error::PlacementError;
use engine::TileMap;

pub fn check(board: &TileMap, tile: &Tile, coordinate: &Coordinate, orientation: &Orientation) -> Result<(), Vec<PlacementError>> {

    let checks: Vec<fn(&TileMap, &Coordinate) -> Option<PlacementError>> =
        vec![check_tile_already_at_coordinate, check_not_adjecent];

    let broken_rules = checks
        .into_iter()
        .filter_map(|rule| rule(board, coordinate))
        .collect::<Vec<_>>();

    if broken_rules.len() > 0 {
        Err(broken_rules)
    } else {
        Ok(())
    }
}

fn check_tile_already_at_coordinate(board: &TileMap,
                                    coordinate: &Coordinate)
                                    -> Option<PlacementError> {
    if board.contains_key(&coordinate) {
        return Some(PlacementError::TileAlreadyAtCoordinate);
    }

    None
}

fn check_not_adjecent(board: &TileMap, coordinate: &Coordinate) -> Option<PlacementError> {
    //////// UNDERENGINEERED
    //////// TODO: OVERENGINEER
    let has_adjecent_tiles = board.contains_key(&[coordinate.x, coordinate.y - 1].into()) ||
                             board.contains_key(&[coordinate.x, coordinate.y + 1].into()) ||
                             board.contains_key(&[coordinate.x + 1, coordinate.y].into()) ||
                             board.contains_key(&[coordinate.x - 1, coordinate.y].into());

    if !has_adjecent_tiles {
        return Some(PlacementError::NotAdjacent);
    }

    None
}


#[cfg(test)]
mod test {
    use super::*;
    use super::super::models::orientation::*;

    #[test]
    fn test_rule_can_place_next_to_tile() {
        let mut board: TileMap = HashMap::new();

        board.insert(Coordinate::from([0, 0]), (Tile::new(), Orientation::Up));

        let next_tile = Tile::new();
        let next_coordinate = Coordinate::from([0, 1]);

        let result = check(&board, &next_tile, &next_coordinate, &Orientation::Up);

        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_rule_cannot_place_with_a_gap() {
        let mut board: TileMap = HashMap::new();

        board.insert(Coordinate::from([0, 0]), (Tile::new(), Orientation::Up));

        let next_tile = Tile::new();
        let next_coordinate = Coordinate::from([0, 2]);

        let result = check(&board, &next_tile, &next_coordinate, &Orientation::Up);

        assert_eq!(result, Err(vec![PlacementError::NotAdjacent]));
    }

    #[test]
    fn test_rule_cannot_place_tiles_in_same_location() {
        let mut board: TileMap = HashMap::new();

        board.insert(Coordinate::from([0, 0]), (Tile::new(), Orientation::Up));

        let next_tile = Tile::new();
        let next_coordinate = Coordinate::from([0, 0]);

        let result = check(&board, &next_tile, &next_coordinate, &Orientation::Up);

        assert!(result.unwrap_err().contains(&PlacementError::TileAlreadyAtCoordinate));
    }

    // TODO: Implement : )
    // #[test]
    // fn test_field_road_field_can_only_connect_to_another_field_road_field() {
    // }
}
