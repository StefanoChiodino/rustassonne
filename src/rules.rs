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
    if engine.tiles.contains(&coordinate) {
        return Some(PlacementError::TileAlreadyAtCoordinate);
    }

    None
}

fn check_not_adjecent(engine: &Engine, coordinate: &Coordinate) -> Option<PlacementError> {
    //////// UNDERENGINEERED
    //////// TODO: OVERENGINEER
    let hasAdjecentTiles = engine
        .tiles
        .contains(&[coordinate.x, coordinate.y - 1].into()) ||
                           engine
                               .tiles
                               .contains(&[coordinate.x, coordinate.y + 1].into()) ||
                           engine
                               .tiles
                               .contains(&[coordinate.x + 1, coordinate.y].into()) ||
                           engine
                               .tiles
                               .contains(&[coordinate.x - 1, coordinate.y].into());

    if !hasAdjecentTiles {
        return Some(PlacementError::NotAdjacent);
    }

    None
}