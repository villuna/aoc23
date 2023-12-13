use std::cmp::min;
use std::collections::HashSet;

use crate::AOContext;

type Coord = (usize, usize);

#[derive(Debug)]
struct Pattern {
    rocks: HashSet<Coord>,
    dim: Coord,
}

impl Pattern {
    fn find_reflection(&self, smudges: usize) -> Option<Reflection> {
        // Try vertical
        'outer: for ref_x in 1..self.dim.0 {
            let mut mistakes = 0;

            for dx in 0..min(ref_x, self.dim.0 - ref_x) {
                let (left, right) = (ref_x - dx - 1, ref_x + dx);
                for (_x, y) in self.rocks.iter().filter(|(x, _y)| *x == left) {
                    if !self.rocks.contains(&(right, *y)) {
                        if mistakes < smudges {
                            mistakes += 1;
                        } else {
                            continue 'outer;
                        }
                    }
                }

                for (_x, y) in self.rocks.iter().filter(|(x, _y)| *x == right) {
                    if !self.rocks.contains(&(left, *y)) {
                        if mistakes < smudges {
                            mistakes += 1
                        } else {
                            continue 'outer;
                        }
                    }
                }
            }

            if mistakes == smudges {
                return Some(Reflection::Vertical(ref_x));
            }
        }

        // Try horizontal
        'outer: for ref_y in 1..self.dim.1 {
            let mut mistakes = 0;

            for dy in 0..min(ref_y, self.dim.1 - ref_y) {
                let (left, right) = (ref_y - dy - 1, ref_y + dy);
                for (x, _y) in self.rocks.iter().filter(|(_x, y)| *y == left) {
                    if !self.rocks.contains(&(*x, right)) {
                        if mistakes < smudges {
                            mistakes += 1;
                        } else {
                            continue 'outer;
                        }
                    }
                }

                for (x, _y) in self.rocks.iter().filter(|(_x, y)| *y == right) {
                    if !self.rocks.contains(&(*x, left)) {
                        if mistakes < smudges {
                            mistakes += 1;
                        } else {
                            continue 'outer;
                        }
                    }
                }
            }

            if mistakes == smudges {
                return Some(Reflection::Horizontal(ref_y));
            }
        }

        None
    }
}

enum Reflection {
    Horizontal(usize),
    Vertical(usize),
}

fn solve(input: &[Pattern], smudges: usize) -> usize {
    input
        .iter()
        .map(|pattern| match pattern.find_reflection(smudges).unwrap() {
            Reflection::Vertical(x) => x,
            Reflection::Horizontal(y) => 100 * y,
        })
        .sum::<usize>()
}

pub fn day13(input: String, ctx: &mut AOContext) {
    let input = input
        .split("\n\n")
        .map(|s| {
            let dim = (s.lines().next().unwrap().len(), s.lines().count());
            let rocks = s
                .lines()
                .enumerate()
                .flat_map(|(y, l)| l.chars().enumerate().map(move |(x, c)| ((x, y), c)))
                .filter_map(|((x, y), c)| if c == '#' { Some((x, y)) } else { None })
                .collect::<HashSet<_>>();

            Pattern { rocks, dim }
        })
        .collect::<Vec<_>>();

    ctx.parsing_done();

    ctx.submit_part1(solve(&input, 0));
    ctx.submit_part2(solve(&input, 1));
}
