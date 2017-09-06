#[allow(unused)]
#[derive(Debug, Hash)]
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
    LeftTop
}