use direction::Direction;

#[derive(Debug)]
pub enum Feature {
    City {
        badge: bool,
        connections: Vec<Direction>,
    },
    Field { connections: Vec<Direction> }
    Monastery,
    Road { connections: Vec<Direction> },
}
