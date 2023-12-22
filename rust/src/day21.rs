use std::collections::{HashMap, VecDeque};

use crate::{
    utils::{Coord, Dir},
    AOContext,
};

#[derive(Debug)]
struct Env<'a> {
    map: &'a [u8],
    start: Coord,
    dim: (isize, isize),
}

impl Env<'_> {
    fn is_rock(&self, Coord(x, y): Coord) -> bool {
        self.map[y as usize * 132 + x as usize] == b'#'
    }
}

fn parse_input(input: &str) -> Env {
    Env {
        map: input.as_bytes(),
        start: Coord(65, 65),
        dim: (131, 131),
    }
}

fn solve(env: &Env, ctx: &mut AOContext) {
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
                if !visited.contains_key(&next) && !env.is_rock(next) {
                    frontier.push_back((dist + 1, next));
                }
            }
        }
    }

    let p1 = visited
        .values()
        .filter(|v| **v <= 64 && **v % 2 == 0)
        .count();

    ctx.submit_part1(p1);

    let even_corners = visited
        .values()
        .filter(|v| **v % 2 == 0 && **v > 65)
        .count();
    let odd_corners = visited
        .values()
        .filter(|v| **v % 2 == 1 && **v > 65)
        .count();

    // This is 202300 but im writing it out here to show the process
    let n = ((26501365 - (env.dim.0 / 2)) / env.dim.0) as usize;
    assert_eq!(n, 202300);

    let even = n * n;
    let odd = (n + 1) * (n + 1);

    let p2 = odd * visited.values().filter(|v| **v % 2 == 1).count()
        + even * visited.values().filter(|v| **v % 2 == 0).count()
        - ((n + 1) * odd_corners)
        + (n * even_corners);

    ctx.submit_part2(p2);
}

pub fn day21(input: String, ctx: &mut AOContext) {
    let env = parse_input(&input);
    ctx.parsing_done();
    solve(&env, ctx);
}
