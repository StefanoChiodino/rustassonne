#[derive(Debug, PartialEq)]
pub enum PlacementError {
    NotAdjacent,
    TileAlreadyAtCoordinate,
}