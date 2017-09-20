use models::feature::Feature;

/// Represents a map square.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Tile {
    features: Vec<Feature>,
}

impl Tile {
    pub fn new() -> Self{
        Tile{features: vec![]}
    }
}