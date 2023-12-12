use nom::{
    bytes::complete::{is_not, tag},
    multi::separated_list1,
    sequence::separated_pair,
    Parser,
};
use rayon::prelude::*;
use std::{collections::HashMap, time::Instant};

use crate::parsers::int;

#[derive(Eq, PartialEq, Debug)]
struct Burst {
    start: i8,
    length: i8,
}

fn possible_solutions<'a, 'b>(
    line: &'a str,
    springs: &'b [i8],
    cache: &mut HashMap<(&'a str, &'b [i8]), usize>,
) -> usize {
    if let Some(answer) = cache.get(&(line, springs)) {
        return *answer;
    }

    if springs.len() != 0 && line.len() < springs[0] as usize {
        return 0;
    }

    if springs.len() == 0 {
        if line.chars().all(|c| c == '.' || c == '?') {
            return 1;
        } else {
            return 0;
        }
    }

    // ...haskell??
    let (x, xs) = (springs[0] as usize, &springs[1..]);
    let mut count = 0;

    for i in 0..=line.len() - x {
        let window = &line[i..i + x];

        if window.chars().all(|c| c == '#' || c == '?')
            && (line.len() == i + x || &line[i + x..i + x + 1] != "#")
        {
            if line.len() == i + x {
                count += possible_solutions("", xs, cache);
            } else {
                count += possible_solutions(&line[i + x + 1..], xs, cache);
            }
        }

        if window.chars().next().unwrap() == '#' {
            break;
        }
    }

    cache.insert((line, springs), count);
    count
}

fn solve(lines: &[(String, Vec<i8>)]) -> usize {
    lines
        .par_iter()
        .map(|(line, springs)| {
            let mut cache = HashMap::new();
            possible_solutions(line, springs, &mut cache)
        })
        .sum::<usize>()
}

fn unfold(lines: &[(String, Vec<i8>)]) -> Vec<(String, Vec<i8>)> {
    lines
        .par_iter()
        .map(|(line, springs)| {
            let mut new_line = line.to_string();
            let mut new_springs = springs.clone();

            for _ in 0..4 {
                new_line.push('?');
                new_line.push_str(line);
                new_springs.extend_from_slice(springs);
            }

            (new_line, new_springs)
        })
        .collect()
}

pub fn day12(input: String) {
    let mut line = separated_pair(
        is_not(" ").map(|s: &str| s.to_string()),
        tag(" "),
        separated_list1(tag(","), int::<i8>),
    );

    let now = Instant::now();

    let lines = input
        .lines()
        .map(|l| line(l).unwrap().1)
        .collect::<Vec<_>>();

    let parsing = now.elapsed().as_secs_f64() * 1000.0;
    let now = Instant::now();

    println!("part 1: {}", solve(&lines));
    
    let p1 = now.elapsed().as_secs_f64() * 1000.0;
    let now = Instant::now();

    let unfolded = unfold(&lines);
    println!("part 2: {}", solve(&unfolded));

    let p2 = now.elapsed().as_secs_f64() * 1000.0;

    println!("parsing took {parsing:.2}ms\npart 1 took {p1:.2}ms\npart 2 took {p2:.2}ms");
}
