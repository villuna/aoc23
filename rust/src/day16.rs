use std::collections::HashSet;
use itertools::Itertools;
use rayon::prelude::*;

use crate::utils::{Dir, add_coords};
use crate::AOContext;

struct Beam {
    dir: Dir,
    pos: (isize, isize),
}

impl Beam {
    fn march(&self, env: &Env, frontier: &mut Vec<Beam>, visited: &mut HashSet<((isize, isize), Dir)>) {
        let increment = self.dir.increment();
        let mut pos = self.pos;

        while let Some(c) = env.get_char(pos) {
            if visited.contains(&(pos, self.dir)) {
                break;
            }

            visited.insert((pos, self.dir));

            match c {
                b'.' => {
                    pos = add_coords(pos, increment);
                },
                b'/' | b'\\' => {
                    let new_dir = if c == b'/' {
                        match self.dir {
                            Dir::Up => Dir::Right,
                            Dir::Down => Dir::Left,
                            Dir::Left => Dir::Down,
                            Dir::Right => Dir::Up,
                        }
                    } else {
                        match self.dir {
                            Dir::Up => Dir::Left,
                            Dir::Down => Dir::Right,
                            Dir::Left => Dir::Up,
                            Dir::Right => Dir::Down,
                        }
                    };

                    frontier.push(Beam { dir: new_dir, pos: add_coords(pos, new_dir.increment())});
                    break;
                },
                b'|' => {
                    if matches!(self.dir, Dir::Up | Dir::Down) {
                        pos = add_coords(pos, increment);
                    } else {
                        frontier.push(Beam { dir: Dir::Up, pos: add_coords(pos, Dir::Up.increment()) });
                        frontier.push(Beam { dir: Dir::Down, pos: add_coords(pos, Dir::Down.increment()) });
                        break;
                    }
                }
                b'-' => {
                    if matches!(self.dir, Dir::Left | Dir::Right) {
                        pos = add_coords(pos, increment);
                    } else {
                        frontier.push(Beam { dir: Dir::Left, pos: add_coords(pos, Dir::Left.increment()) });
                        frontier.push(Beam { dir: Dir::Right, pos: add_coords(pos, Dir::Right.increment()) });
                        break;
                    }
                }
                _ => {
                    unreachable!("encountered forbidden character {c}");
                },
            }
        }
    }
}

struct Env {
    map: Vec<Vec<u8>>,
    dimensions: (isize, isize),
}

impl Env {
    fn parse(input: &str) -> Self {
        let map = input.lines().map(|line| line.as_bytes().to_vec()).collect::<Vec<_>>();
        let dimensions = (map[0].len() as _, map.len() as _);

        Env {
            map,
            dimensions
        }
    }

    fn is_in_bounds(&self, pos: (isize, isize)) -> bool {
        pos.0 >= 0 && pos.1 >= 0 && pos.0 < self.dimensions.0 && pos.1 < self.dimensions.1
    }

    fn get_char(&self, pos: (isize, isize)) -> Option<u8> {
        self.is_in_bounds(pos).then(|| self.map[pos.1 as usize][pos.0 as usize])
    }
}

fn solve(env: &Env, pos: (isize, isize), dir: Dir) -> usize {
    let mut visited = HashSet::new();
    let mut frontier = vec![Beam { dir, pos }];

    while let Some(beam) = frontier.pop() {
        beam.march(&env, &mut frontier, &mut visited);
    }

    visited.iter().map(|p| p.0).unique().count()
}

pub fn day16(input: String, ctx: &mut AOContext) {
    let env = Env::parse(&input);

    ctx.parsing_done();
    ctx.submit_part1(solve(&env, (0, 0), Dir::Right));

    let p2 = [Dir::Left, Dir::Right, Dir::Up, Dir::Down].into_iter().flat_map(|dir| {
        let pos = match dir {
            Dir::Down => (0..env.dimensions.0).map(|x| (x, 0)).collect::<Vec<_>>(),
            Dir::Left => (0..env.dimensions.1).map(|y| (env.dimensions.1 - 1, y)).collect::<Vec<_>>(),
            Dir::Up => (0..env.dimensions.0).map(|x| (x, env.dimensions.0 - 1)).collect::<Vec<_>>(),
            Dir::Right => (0..env.dimensions.1).map(|y| (0, y)).collect::<Vec<_>>()
        };
        pos.into_iter().map(move |p| (p, dir))
    }).collect::<Vec<_>>().into_par_iter().map(|(pos, dir)| solve(&env, pos, dir)).max().unwrap();

    ctx.submit_part2(p2);
}
