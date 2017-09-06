#[derive(PartialEq, Eq)]
pub struct Coordinate {
    x: i32,
    y: i32,
}

impl From<[i32; 2]> for Coordinate {
    fn from(point: [i32; 2]) -> Self {
        Coordinate {
            x: point[0],
            y: point[1],
        }
    }
}
