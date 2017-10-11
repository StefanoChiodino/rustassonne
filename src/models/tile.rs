use models::direction::Direction;
use models::feature::Feature;

/// Represents a map square.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Tile {
    pub features: Vec<Feature>,
}

impl Tile {
    pub fn new() -> Self {
        Tile{features: vec![]}
    }

    pub fn features_at_connection(&self, direction: &Direction) -> Vec<Feature> {
        self.features
            .iter()
            .filter(|feature| match **feature {
                // Feature::City { connections }
                // | Feature::Field {connections }
                Feature::Road {connections} => connections.contains(direction),
                _ => false,
            })
            .cloned()
            .collect()
    }

    // pub fn features_at_connection_with_orientation(&self, direction: &Direction, orientation: &Orientation) -> Vec<Feature> {

    // }
}

