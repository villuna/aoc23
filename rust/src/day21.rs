use std::collections::{HashSet, VecDeque, HashMap};

use crate::{AOContext, utils::{Coord, Dir}};

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
                if !visited.contains_key(&next) && !env.rocks.contains(&next) {
                    frontier.push_back((dist + 1, next));
                }
            }
        }
    }

    let p1 = visited.values().filter(|v| **v <= 64 && **v % 2 == 0).count();
    ctx.submit_part1(p1);
    let even_corners = visited.values().filter(|v| **v % 2 == 0 && **v > 65).count();
    let odd_corners = visited.values().filter(|v| **v % 2 == 1 && **v > 65).count();

    // This is 202300 but im writing it out here to show the process
    let n = ((26501365 - (env.dim.0 / 2)) / env.dim.0) as usize;
    assert_eq!(n, 202300);

    let even = n*n;
    let odd = (n+1) * (n+1);

    let p2 = odd * visited.values().filter(|v| **v % 2 == 1).count() + even * visited.values().filter(|v| **v % 2 == 0).count()
        - ((n + 1) * odd_corners) + (n * even_corners);

    ctx.submit_part2(p2);
}

pub fn day21(input: String, ctx: &mut AOContext) {
    let env = parse_input(&input);
    ctx.parsing_done();
    solve(&env, ctx);
}
