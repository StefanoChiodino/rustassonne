use direction::Direction;

#[derive(Debug)]
pub enum Feature {
    Monastery,
    FuckYou,
    Road { connections: Vec<Direction> },
    City {
        badge: bool,
        connections: Vec<Direction>,
    },
}
