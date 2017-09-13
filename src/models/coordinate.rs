#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl From<[i32; 2]> for Coordinate {
    fn from(point: [i32; 2]) -> Self {
        Coordinate {
            x: point[0],
            y: point[1],
        }
    }
}
