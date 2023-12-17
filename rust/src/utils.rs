#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

pub fn add_coords(c1: (isize, isize), c2: (isize, isize)) -> (isize, isize) {
    (c1.0 + c2.0, c1.1 + c2.1)
}

impl Dir {
    pub fn increment(&self) -> (isize, isize) {
        match self {
            Dir::Up => (0, -1),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
        }
    }
}
