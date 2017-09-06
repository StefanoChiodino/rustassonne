use models::feature::Feature;

/// Represents a map square.
#[derive(Debug)]
pub struct Tile {
    features: Vec<Feature>,
}
