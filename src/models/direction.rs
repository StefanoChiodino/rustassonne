use models::orientation::Orientation;
use std::mem::transmute;

/// Represents edges of a Tile where Features connect.
#[allow(unused)]
#[repr(i32)]
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
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

const LENGTH: i32 = 12;

impl Direction {
    pub fn rotate(self, orientation: Orientation) -> Self {
        let integer = (self as i32 + (orientation as i32 * 3)) % LENGTH;

        unsafe { transmute(integer) }
    }

    pub fn opposite(self) -> Self {
        self.roate(Orientation::Down)
    }
}