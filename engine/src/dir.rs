#[derive(Clone, Copy, PartialEq)]
pub enum Dir {
    Right,
    Left,
    Up,
    Down,
    RightDown,
    RightUp,
    LeftDown,
    LeftUp,
}

impl From<(isize, isize)> for Dir {
    fn from(position: (isize, isize)) -> Self {
        match position {
            (0, 1) => Self::Right,
            (0, -1) => Self::Left,
            (-1, 0) => Self::Up,
            (1, 0) => Self::Down,
            (1, 1) => Self::RightDown,
            (-1, 1) => Self::RightUp,
            (1, -1) => Self::LeftDown,
            (-1, -1) => Self::LeftUp,
            _ => unreachable!(),
        }
    }
}

impl From<Dir> for (isize, isize) {
    fn from(val: Dir) -> Self {
        match val {
            Dir::Right => (0, 1),
            Dir::Left => (0, -1),
            Dir::Up => (-1, 0),
            Dir::Down => (1, 0),
            Dir::RightDown => (1, 1),
            Dir::RightUp => (-1, 1),
            Dir::LeftDown => (1, -1),
            Dir::LeftUp => (-1, -1),
        }
    }
}
