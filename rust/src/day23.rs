use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::utils::{Coord, Dir};
use crate::AOContext;
use bit_set::BitSet;

use rayon::prelude::*;

#[derive(Debug)]
struct Env<'a> {
    map: &'a [u8],
    start: Coord,
    goal: Coord,
    dim: (isize, isize),
}

impl Env<'_> {
    fn tile_at(&self, Coord(x, y): Coord) -> Option<u8> {
        (x >= 0 && x < self.dim.0 && y >= 0 && y < self.dim.1)
            .then(|| self.map[y as usize * (self.dim.0 + 1) as usize + x as usize])
    }
}

fn parse_input(input: &str) -> Env {
    let dim = (
        input
            .as_bytes()
            .split(|b| *b == b'\n')
            .next()
            .unwrap()
            .len() as _,
        input
            .as_bytes()
            .split(|b| *b == b'\n')
            .filter(|l| l.len() != 0)
            .count() as _,
    );

    Env {
        map: input.as_bytes(),
        start: Coord(1, 0),
        dim,
        goal: Coord(dim.0 - 2, dim.1 - 1),
    }
}

fn coord_index(env: &Env, coord: Coord) -> usize {
    (coord.1 * env.dim.0 + coord.0) as usize
}

fn part1_helper(env: &Env, current_length: usize, start: Coord, visited: &mut BitSet) -> usize {
    if start == env.goal {
        return current_length;
    }

    visited.insert(coord_index(env, start));

    let next_targets = [Dir::Up, Dir::Down, Dir::Left, Dir::Right]
        .into_iter()
        .filter_map(|d| {
            let coord = start + d;
            if visited.contains(coord_index(env, coord)) {
                None
            } else {
                let c = env.tile_at(coord)?;

                if c == b'#' {
                    None
                } else if b"><^v".contains(&c) {
                    let traversable = (d == Dir::Up && c == b'^')
                        || (d == Dir::Down && c == b'v')
                        || (d == Dir::Left && c == b'<')
                        || (d == Dir::Right && c == b'>');

                    traversable.then_some((2, coord + d))
                } else {
                    Some((1, coord))
                }
            }
        })
        .collect_vec();

    let mut max = current_length;

    for (dist, t) in next_targets {
        let next = part1_helper(env, current_length + dist, t, visited);

        if next > max {
            max = next;
        }
    }

    max
}

fn part1(env: &Env) -> usize {
    let mut visited = BitSet::with_capacity((env.dim.0 * env.dim.1) as usize);
    part1_helper(env, 0, env.start, &mut visited)
}

fn part2(env: &Env) -> usize {
    let mut multi_connected_nodes = vec![env.start, env.goal];

    for x in 0..env.dim.0 {
        for y in 0..env.dim.1 {
            let pos = Coord(x, y);
            let c = env.tile_at(pos).unwrap();

            if c == b'#' {
                continue;
            }

            let open_dirs = [Dir::Up, Dir::Down, Dir::Left, Dir::Right]
                .into_iter()
                .filter(|d| env.tile_at(pos + *d).is_some_and(|c| c != b'#'))
                .count();

            if open_dirs > 2 {
                multi_connected_nodes.push(pos);
            }
        }
    }

    let mut graph = HashMap::<Coord, Vec<(Coord, usize)>>::new();

    for &n in &multi_connected_nodes {
        let mut connections = Vec::new();
        let open_dirs = [Dir::Up, Dir::Down, Dir::Left, Dir::Right]
            .into_iter()
            .filter(|d| env.tile_at(n + *d).is_some_and(|c| c != b'#'));

        for mut d in open_dirs {
            let mut current = n + d;
            let mut dist = 1;

            loop {
                if multi_connected_nodes.contains(&current) {
                    break;
                }

                let open_dirs = [Dir::Up, Dir::Down, Dir::Left, Dir::Right]
                    .into_iter()
                    .filter(|next_d| {
                        *next_d != d.opposite()
                            && env.tile_at(current + *next_d).is_some_and(|c| c != b'#')
                    })
                    .collect_vec();

                assert_eq!(open_dirs.len(), 1);

                d = open_dirs[0];
                current = current + d;
                dist += 1;
            }

            connections.push((current, dist));
        }

        graph.insert(n, connections);
    }

    let mut visited = HashSet::new();
    p2_helper(&graph, env.start, env.goal, 0, &mut visited)
}

fn p2_helper(
    graph: &HashMap<Coord, Vec<(Coord, usize)>>,
    start: Coord,
    goal: Coord,
    dist: usize,
    visited: &mut HashSet<Coord>,
) -> usize {
    if start == goal {
        return dist;
    }

    visited.insert(start);

    if let Some(d) = graph.get(&start).unwrap().iter().find_map(|(n, d)| (*n == goal).then_some(*d)) {
        return dist + d;
    }

    graph.get(&start).unwrap().par_iter().filter_map(|&(node, d)| {
        if visited.contains(&node) {
            return None;
        }
        let mut visited = visited.clone();
        Some(p2_helper(graph, node, goal, dist + d, &mut visited))
    }).max().unwrap_or(0)
}

pub fn day23(input: String, ctx: &mut AOContext) {
    let env = parse_input(&input);
    ctx.parsing_done();
    ctx.submit_part1(part1(&env));
    ctx.submit_part2(part2(&env));
}
