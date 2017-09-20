use direction::Direction;
use std::collections::HashSet;

/// Game features of a tile used for rule enforcement and score counting.
/// Althought some features connect, this isn't required.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Feature {
    City {
        badge: bool,
        connections: HashSet<Direction>,
    },
    Field { connections: HashSet<Direction> },
    Monastery,
    Road { connections: HashSet<Direction> },
}
