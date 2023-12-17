use crate::utils::{add_coords, Dir};
use itertools::Itertools;
use priority_queue::PriorityQueue;
use std::{cmp::Reverse, collections::HashSet, hash::Hash};

use crate::AOContext;

#[derive(Debug, Eq, Clone, Copy)]
struct Entry {
    pos: (isize, isize),
    dir: Option<Dir>,
}

fn dir_axis(dir: Dir) -> u8 {
    match dir {
        Dir::Up | Dir::Down => 0,
        Dir::Left | Dir::Right => 1,
    }
}

impl Hash for Entry {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
        self.dir.map(dir_axis).hash(state);
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.dir.map(dir_axis) == other.dir.map(dir_axis)
    }
}

fn next<const MIN: isize, const MAX: isize>(
    entry: &Entry,
    map: &[&[u8]],
    cost: usize,
) -> Vec<(usize, Entry)> {
    let dim = (map[0].len() as isize, map.len() as isize);

    let directions: &[Dir] = match entry.dir {
        None => &[Dir::Up, Dir::Down, Dir::Left, Dir::Right],
        Some(Dir::Up) | Some(Dir::Down) => &[Dir::Left, Dir::Right],
        Some(Dir::Left) | Some(Dir::Right) => &[Dir::Up, Dir::Down],
    };

    directions
        .into_iter()
        .flat_map(|d| {
            let mut v = vec![];
            let mut pos = entry.pos;
            let mut cost = cost;
            for i in 0..MAX {
                pos = add_coords(pos, d.increment());
                if !(pos.0 >= 0 && pos.1 >= 0 && pos.0 < dim.0 && pos.1 < dim.1) {
                    break;
                }
                cost += (map[pos.1 as usize][pos.0 as usize] - b'0') as usize;

                // stupid hack to get this to work with my part 1 soln
                if i < MIN {
                    continue;
                }
                v.push((cost, Entry { pos, dir: Some(*d) }));
            }
            v
        })
        .collect_vec()
}

fn solve(map: &[&[u8]], edge_fn: impl Fn(&Entry, &[&[u8]], usize) -> Vec<(usize, Entry)>) -> usize {
    let mut frontier = PriorityQueue::new();
    let mut visited = HashSet::new();
    for dir in [Dir::Right, Dir::Left, Dir::Down, Dir::Up] {
        visited.insert(Entry {
            pos: (0, 0),
            dir: Some(dir),
        });
    }

    let goal = (map[0].len() as isize - 1, map.len() as isize - 1);
    frontier.push(
        Entry {
            pos: (0, 0),
            dir: None,
        },
        Reverse(0),
    );

    while let Some((node, Reverse(cost))) = frontier.pop() {
        if node.pos == goal {
            return cost;
        }

        visited.insert(node);
        let next = edge_fn(&node, map, cost);

        for (next_cost, next_node) in next {
            if !visited.contains(&next_node) {
                frontier.push_increase(next_node, Reverse(next_cost));
            }
        }
    }

    panic!("didnt find any path!");
}

pub fn day17(input: String, ctx: &mut AOContext) {
    let map = input.lines().map(|l| l.as_bytes()).collect_vec();

    ctx.submit_part1(solve(&map, next::<0, 3>));
    ctx.submit_part2(solve(&map, next::<3, 10>));
}
