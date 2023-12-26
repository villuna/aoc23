use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

use crate::AOContext;

pub fn day25(input: String, ctx: &mut AOContext) {
    /*
    println!("strict graph g {{");
    for line in input.lines() {
        let (name, cxs) = line.split(": ").collect_tuple().unwrap();
        for c in cxs.split(" ") {
            println!("{name} -- {c};");
        }
    }
    println!("}}");
    */

    let snip = [
        ("fdb", "psj"),
        ("nqh", "rmt"),
        ("ltn", "trh"),
    ];

    let mut graph = HashMap::<&str, Vec<&str>>::new();

    for line in input.lines() {
        let (name, cxs) = line.split(": ").collect_tuple().unwrap();
        for c in cxs.split(" ") {
            if snip.contains(&(name, c)) || snip.contains(&(c, name)) {
                continue;
            }
            graph.entry(name).or_insert_with(|| vec![]).push(c);
            graph.entry(c).or_insert_with(|| vec![]).push(name);
        }
    }

    let mut sizes = [0, 0];

    for (i, s) in ["fdb", "psj"].into_iter().enumerate() {
        let mut visited = HashSet::<&str>::new();
        let mut frontier = VecDeque::new();
        frontier.push_back(s);

        while let Some(n) = frontier.pop_front() {
            if !visited.insert(n) {
                continue;
            }

            for next in graph[n].iter() {
                if !visited.contains(next) {
                    frontier.push_back(next);
                }
            }
        }

        sizes[i] = visited.len();
    }

    ctx.submit_both(sizes[0] * sizes[1], "merry christmas!");
}
