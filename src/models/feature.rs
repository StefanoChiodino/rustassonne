use direction::Direction;
use std::collections::HashSet;

#[derive(Debug)]
pub enum Feature {
    City {
        badge: bool,
        connections: HashSet<Direction>,
    },
    Field { connections: HashSet<Direction> },
    Monastery,
    Road { connections: HashSet<Direction> },
}
