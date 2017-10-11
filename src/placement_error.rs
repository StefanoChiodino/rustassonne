#[derive(Debug, PartialEq)]
pub enum PlacementError {
    EdgeMismatch,
    NotAdjacent,
    TileAlreadyAtCoordinate,
}