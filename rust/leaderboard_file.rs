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
}

use std::collections::{HashSet, VecDeque, HashMap};

#[derive(Debug)]
struct Env {
    rocks: HashSet<Coord>,
    start: Coord,
    dim: (isize, isize),
}

fn parse_input(input: &str) -> Env {
    let mut rocks = HashSet::new();
    let mut start = Coord(0, 0);
    let mut dim = (0, 0);

    for (y, line) in input.as_bytes().split(|b| *b == b'\n').enumerate() {
        if line.len() == 0 {
            continue;
        }
        dim.1 = y;
        for (x, c) in line.iter().enumerate() {
            dim.0 = x;
            match *c {
                b'#' => { rocks.insert(Coord(x as _, y as _)); },
                b'S' => { start = Coord(x as _, y as _); },
                _ => {},
            }
        }
    }

    let dim = (dim.0 as isize + 1, dim.1 as isize + 1);

    Env {
        rocks,
        start,
        dim
    }
}

fn p2(env: &Env) -> usize {
    let mut frontier = VecDeque::<(usize, Coord)>::new();
    let mut visited = HashMap::new();
    frontier.push_back((0, env.start));

    while let Some((dist, coord)) = frontier.pop_front() {
        if visited.contains_key(&coord) {
            continue;
        }
        
        visited.insert(coord, dist);

        for d in [Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
            let inc = d.increment();
            let next = coord + Coord(inc.0, inc.1);

            if next.0 >= 0 && next.0 < env.dim.0 && next.1 >= 0 && next.1 < env.dim.1 {
                if !visited.contains_key(&next) && !env.rocks.contains(&next) {
                    frontier.push_back((dist + 1, next));
                }
            }
        }
    }

    let even_corners = visited.values().filter(|v| **v % 2 == 0 && **v > 65).count();
    let odd_corners = visited.values().filter(|v| **v % 2 == 1 && **v > 65).count();

    // This is 202300 but im writing it out here to show the process
    let n = ((26501365 - (env.dim.0 / 2)) / env.dim.0) as usize;

    let even = n*n;
    let odd = (n+1) * (n+1);

    let p2 = odd * visited.values().filter(|v| **v % 2 == 1).count() + even * visited.values().filter(|v| **v % 2 == 0).count()
        - ((n + 1) * odd_corners) + (n * even_corners);

    p2
}

pub fn run(input: &str) -> impl std::fmt::Display {
    let env = parse_input(&input);
    p2(&env)
}
