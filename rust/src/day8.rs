use std::{collections::HashMap, time::Instant};

use nom::{
    bytes::complete::{is_a, tag},
    multi::separated_list1,
    sequence::{delimited, pair, separated_pair, terminated},
    IResult, Parser,
};

use crate::parsers::newline;

#[derive(Debug)]
struct Environment<'a> {
    path: Vec<usize>,
    graph: HashMap<&'a str, [&'a str; 2]>,
}

fn parse(input: &str) -> IResult<&str, Environment> {
    let header = terminated(is_a("LR"), newline);
    let graph_entry = separated_pair(
        is_a("ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
        tag(" = "),
        delimited(
            tag("("),
            separated_pair(
                is_a("ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
                tag(", "),
                is_a("ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
            ).map(|(l, r)| [l, r]),
            tag(")"),
        ),
    );

    let graph = separated_list1(newline, graph_entry);

    pair(header, graph)
        .map(|(path, graph)| Environment {
            path: path
                .chars()
                .map(|c| match c {
                    'L' => 0,
                    'R' => 1,
                    _ => unreachable!(),
                })
                .collect(),
            graph: graph.iter().cloned().collect(),
        })
        .parse(input)
}

pub fn day8(input: String) {
    let mut now = Instant::now();
    let env = parse(&input).unwrap().1;
    let parsing = now.elapsed().as_secs_f64() * 1000.0;
    println!("parsing took {parsing:.2}ms");

    now = Instant::now();
    let p1 = general_solution(&env, |&node| node == "AAA", |&node| node == "ZZZ");
    let part1 = now.elapsed().as_secs_f64() * 1000.0;

    println!("part 1: {p1} (took {part1:.2}ms)");

    now = Instant::now();
    let p2 = general_solution(
        &env,
        |node| node.chars().last() == Some('A'),
        |node| node.chars().last() == Some('Z'),
    );
    let part2 = now.elapsed().as_secs_f64() * 1000.0;
    println!("part 2: {p2} (took {part2:.2}ms)");
}

fn general_solution(
    env: &Environment,
    is_start_state: impl Fn(&&str) -> bool,
    is_end_state: impl Fn(&&str) -> bool,
) -> u64 {
    let mut path_map: HashMap<&str, Vec<&str>> = HashMap::new();

    for &node in env.graph.keys() {
        if is_end_state(&node) {
            path_map.insert("node", vec![]);
            continue;
        }

        let mut current = node;
        let mut path = vec![];

        for dir in env.path.iter() {
            current = &env.graph[current][*dir];

            path.push(current);
            if is_end_state(&current) {
                break;
            }
        }

        path_map.insert(node, path);
    }

    let res = env
        .graph
        .keys()
        .cloned()
        .filter(is_start_state)
        .map(|node| {
            let mut steps = 0u64;
            let mut current = node;

            loop {
                let path = path_map.get(current).unwrap();
                steps += path.len() as u64;

                if let Some(dest) = path.last() {
                    current = *dest;

                    if is_end_state(&current) {
                        break;
                    }
                } else {
                    break;
                }
            }

            steps
        })
        .fold(1, |a, b| num::integer::lcm(a, b));

    res
}
