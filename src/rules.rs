use std::collections::HashMap;

use models::coordinate::Coordinate;
use models::orientation::Orientation;
use models::direction::Direction;
use models::tile::Tile;
use placement_error::PlacementError;
use engine::TileMap;
use std::rc::Rc;

pub fn check(
    board: &TileMap,
    tile: &Tile,
    coordinate: &Coordinate,
    orientation: &Orientation) -> Result<(), Vec<PlacementError>> {

    let checks: Vec<fn(&TileMap, &Tile, &Coordinate, &Orientation) -> Option<PlacementError>> = vec![
        check_tile_already_at_coordinate,
        check_not_adjecent,
        check_edges_match,
    ];

    let broken_rules = checks
        .into_iter()
        .filter_map(|rule| rule(board, tile, coordinate, orientation))
        .collect::<Vec<_>>();

    if broken_rules.len() > 0 {
        Err(broken_rules)
    } else {
        Ok(())
    }
}

fn check_tile_already_at_coordinate(
    board: &TileMap,
    tile: &Tile,
    coordinate: &Coordinate,
    orientation: &Orientation)
    -> Option<PlacementError> {
    if board.contains_key(&coordinate) {
        return Some(PlacementError::TileAlreadyAtCoordinate);
    }

    None
}

fn check_not_adjecent(
    board: &TileMap,
    tile: &Tile,
    coordinate: &Coordinate,
    orientation: &Orientation)
    -> Option<PlacementError> {
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

fn check_edges_match(
    board: &TileMap,
    tile: &Tile,
    coordinate: &Coordinate,
    orientation: &Orientation)
    -> Option<PlacementError> {
    // Go to each adjacent tile for given coordinate, and check that the
    // neighbouring ediges have the same feature profile.

    let all_adjacent_side_and_tile_placements = [
        board.get(&[coordinate.x, coordinate.y - 1].into()).map(|&(ref tile, ref orientation)| (Direction::Top, tile.clone(), orientation.clone())),
        // (&Direction::Bottom, board.get(&[coordinate.x, coordinate.y + 1].into())),
        // (&Direction::Right,  board.get(&[coordinate.x + 1, coor vdinate.y].into())),
        // (&Direction::Left,   board.get(&[coordinate.x - 1, coordinate.y].into())),
    ];

    let adjacent_side_and_tile_placements: Vec<(Direction, Rc<Tile>, Orientation)> = all_adjacent_side_and_tile_placements
        .iter()
        .cloned()
        .filter_map(|adjacent| adjacent)
        .collect();

    // for &(ref direction, ref tile, ref orientation) in adjacent_side_and_tile_placements.iter() {
    //     match direction {
    //         Direction::Top => tile.features.contains()
    //     }
    // }

    adjacent_side_and_tile_placements
        .iter()
        .map(|&(ref direction, ref adjacent_tile, ref adjacent_orientation)| match direction {
            // rotate the original
            // get triplet of the rotated direction
            // get triplet of the opposite of the original direction
            let rotated_direction = direction.rotate(orientation);
            let adjacent_rotated_direction = direction.opposite().rotate(adjacent_orientation);

            // rotated_direction.triplet();
            // adjacent_rotated_direction.triplet();

            // Map them to the features then Zip the triplets to create something like this?:
            // [
            //     (tile.features_at_connection(Direction::TopLeft), adjacent.features_at_connection(Direction::BottomLeft)),
            //     (tile.features_at_connection(Direction::Top), adjacent.features_at_connection(Direction::Bottom)),
            //     (tile.features_at_connection(Direction::TopRight), adjacent.features_at_connection(Direction::BottomRight)),
            // ]
        })

    None
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::models::orientation::*;

    #[test]
    fn test_rule_can_place_next_to_tile() {
        let mut board: TileMap = HashMap::new();

        board.insert(Coordinate::from([0, 0]), (Rc::new(Tile::new()), Orientation::Up));

        let next_tile = Tile::new();
        let next_coordinate = Coordinate::from([0, 1]);

        let result = check(&board, &next_tile, &next_coordinate, &Orientation::Up);

        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_rule_cannot_place_with_a_gap() {
        let mut board: TileMap = HashMap::new();

        board.insert(Coordinate::from([0, 0]), (Rc::new(Tile::new()), Orientation::Up));

        let next_tile = Tile::new();
        let next_coordinate = Coordinate::from([0, 2]);

        let result = check(&board, &next_tile, &next_coordinate, &Orientation::Up);
        let err = result.unwrap_err();

        assert!(err.contains(&PlacementError::NotAdjacent), err);
    }

    #[test]
    fn test_rule_cannot_place_tiles_in_same_location() {
        let mut board: TileMap = HashMap::new();

        board.insert(Coordinate::from([0, 0]), (Rc::new(Tile::new()), Orientation::Up));

        let next_tile = Tile::new();
        let next_coordinate = Coordinate::from([0, 0]);

        let result = check(&board, &next_tile, &next_coordinate, &Orientation::Up);
        let err = result.unwrap_err();

        assert!(err.contains(&PlacementError::TileAlreadyAtCoordinate), err);
    }

    #[test]
    fn test_field_road_field_edge_connects_to_another_field_road_field_edge_by_placement() {
        use direction::Direction;
        use super::super::tile_builder::TileBuilder;

        let mut board: TileMap = HashMap::new();

        let top = TileBuilder::new()
            .with_field(&[Direction::TopLeft, Direction::TopRight])
            .with_road(&[Direction::Top])
            .build();

        let bottom = TileBuilder::new()
            .with_road(&[Direction::Bottom])
            .build();

        board.insert(Coordinate::from([0, 1]), (Rc::new(top), Orientation::Up));

        let result = check(&board, &bottom, &Coordinate::from([0, 0]), &Orientation::Up);
        assert_eq!(result, Ok(()));
    }
    
    #[test]
    fn test_field_road_field_edge_does_not_connects_to_another_field_field_field_edge_by_placement() {
        use direction::Direction;
        use super::super::tile_builder::TileBuilder;

        let mut board: TileMap = HashMap::new();

        let top = TileBuilder::new()
            .with_field(&[Direction::RightTop, Direction::RightBottom])
            .with_road(&[Direction::Right])
            .build();

        let bottom = TileBuilder::new()
            .with_field(&[Direction::BottomLeft, Direction::BottomRight])
            .with_road(&[Direction::Bottom])
            .build();

        board.insert(Coordinate::from([0, 1]), (Rc::new(top), Orientation::Up));

        let result = check(&board, &bottom, &Coordinate::from([0, 0]), &Orientation::Up);
        let err = result.unwrap_err();
        assert!(err.contains(&PlacementError::EdgeMismatch), err);
    }
}
