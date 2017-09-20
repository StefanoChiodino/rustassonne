use models::coordinate::Coordinate;
use engine::Engine;
use placement_error::PlacementError;

pub fn check(engine: &Engine, coordinate: &Coordinate) -> Result<(), Vec<PlacementError>> {

    let checks: Vec<fn(&Engine, &Coordinate) -> Option<PlacementError>> =
        vec![check_tile_already_at_coordinate, check_not_adjecent];

    let broken_rules = checks
        .into_iter()
        .filter_map(|rule| rule(engine, coordinate))
        // .filter(|error_option| error_option.is_some())
        .collect::<Vec<_>>();

    if broken_rules.len() > 0 {
        Err(broken_rules)
    } else {
        Ok(())
    }
}

fn check_tile_already_at_coordinate(engine: &Engine,
                                    coordinate: &Coordinate)
                                    -> Option<PlacementError> {
    if engine.get_tiles().contains_key(&coordinate) {
        return Some(PlacementError::TileAlreadyAtCoordinate);
    }

    None
}

fn check_not_adjecent(engine: &Engine, coordinate: &Coordinate) -> Option<PlacementError> {
    //////// UNDERENGINEERED
    //////// TODO: OVERENGINEER
    let has_adjecent_tiles = engine
        .get_tiles()
        .contains_key(&[coordinate.x, coordinate.y - 1].into()) ||
                             engine
                                 .get_tiles()
                                 .contains_key(&[coordinate.x, coordinate.y + 1].into()) ||
                             engine
                                 .get_tiles()
                                 .contains_key(&[coordinate.x + 1, coordinate.y].into()) ||
                             engine
                                 .get_tiles()
                                 .contains_key(&[coordinate.x - 1, coordinate.y].into());

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
    fn test_rule_cannot_place_on_center() {
        let engine = Engine::new();
        let result = check(&engine, &[0, 0].into());

        assert_eq!(result,
                   Err(vec![PlacementError::TileAlreadyAtCoordinate, PlacementError::NotAdjacent]));
    }

    #[test]
    fn test_rule_can_place_next_to_center() {
        let engine = Engine::new();
        let result = check(&engine, &[0, 1].into());

        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_rule_cannot_place_with_a_gap() {
        let engine = Engine::new();

        let result = check(&engine, &[0, 2].into());

        assert_eq!(result, Err(vec![PlacementError::NotAdjacent]));
    }

    #[test]
    fn test_rule_cannot_place_tiles_in_same_location() {
        let mut engine = Engine::new();
        engine = engine.place_next([0, 1], Orientation::Up).unwrap();

        let result = check(&engine, &[0, 1].into());

        assert_eq!(result, Err(vec![PlacementError::TileAlreadyAtCoordinate]));
    }
}
