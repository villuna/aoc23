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

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct Coord(pub isize, pub isize);

impl std::ops::Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Add<Dir> for Coord {
    type Output = Self;

    fn add(self, rhs: Dir) -> Self::Output {
        let rhs = rhs.increment();
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Mul<isize> for Coord {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Coord(self.0 * rhs, self.1 * rhs)
    }
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

    pub fn cincrement(&self) -> Coord {
        match self {
            Dir::Up => Coord(0, -1),
            Dir::Down => Coord(0, 1),
            Dir::Left => Coord(-1, 0),
            Dir::Right => Coord(1, 0),
        }
    }

    pub fn from_char(c: char) -> Dir {
        match c {
            'U' => Dir::Up,
            'D' => Dir::Down,
            'L' => Dir::Left,
            'R' => Dir::Right,
            _ => panic!(),
        }
    }

    pub fn opposite(&self) -> Dir {
        match self {
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
        }
    }
}
