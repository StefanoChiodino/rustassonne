/// Represents edges of a Tile where Features connect.
#[allow(unused)]
#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Direction {
    TopLeft,
    Top,
    TopRight,

    RightTop,
    Right,
    RightBottom,

    BottomRight,
    Bottom,
    BottomLeft,

    LeftBottom,
    Left,
    LeftTop,
}
