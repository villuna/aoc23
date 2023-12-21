use std::{
    collections::{HashMap, VecDeque},
    convert::identity,
};

use nom::{
    bytes::complete::{is_a, tag},
    character::complete::one_of,
    combinator::opt,
    multi::separated_list1,
    sequence::{pair, separated_pair},
    IResult, Parser,
};
use num::Integer;

use crate::AOContext;

enum ModuleType<'a> {
    FlipFlop(bool),
    Conj(HashMap<&'a str, bool>),
    Basic,
}

struct Module<'a> {
    ty: ModuleType<'a>,
    destinations: Vec<&'a str>,
}

fn name(input: &str) -> IResult<&str, &str> {
    is_a("abcdefghijklmnopqrstuvwxyz")(input)
}

fn module(input: &str) -> IResult<&str, (&str, (Option<char>, Vec<&str>))> {
    separated_pair(
        pair(opt(one_of("&%")), name),
        tag(" -> "),
        separated_list1(tag(", "), name),
    )
    .map(|((c, n), d)| (n, (c, d)))
    .parse(input)
}

pub fn day20(input: String, ctx: &mut AOContext) {
    let input: HashMap<&str, (Option<char>, Vec<&str>)> =
        HashMap::from_iter(input.lines().map(|s| module(s).unwrap().1));

    let mut modules: HashMap<&str, Module> = HashMap::new();

    for (name, (ty, dests)) in input.clone().into_iter() {
        let ty = match ty {
            None => ModuleType::Basic,
            Some('%') => ModuleType::FlipFlop(false),
            Some('&') => ModuleType::Conj({
                HashMap::from_iter(
                    input
                        .iter()
                        .filter(|(_k, v)| v.1.contains(&name))
                        .map(|(&k, _v)| (k, false)),
                )
            }),
            _ => unreachable!(),
        };

        modules.insert(
            name,
            Module {
                ty,
                destinations: dests,
            },
        );
    }

    ctx.parsing_done();

    let (p1, p2) = solve(modules);
    ctx.submit_both(p1, p2);
}

fn solve(mut modules: HashMap<&str, Module>) -> (usize, usize) {
    let mut low = 0;
    let mut high = 0;
    let mut queue = VecDeque::new();
    let mut i = 0;

    let target = modules
        .iter()
        .find_map(|(k, v)| v.destinations.contains(&"rx").then_some(*k))
        .unwrap();

    let mut cycles: HashMap<&str, usize> = {
        let ModuleType::Conj(mem) = &modules.get(target).unwrap().ty else { unreachable!() };
        HashMap::from_iter(mem.iter().map(|(k, _)| (*k, 0)))
    };

    'outer: loop {
        i += 1;
        queue.push_back((false, "button", "broadcaster"));

        while let Some((pulse, input, module_name)) = queue.pop_front() {
            if i <= 1000 {
                if pulse {
                    high += 1;
                } else {
                    low += 1;
                }
            }

            let Some(module) = modules.get_mut(module_name) else { continue };

            match &mut module.ty {
                ModuleType::FlipFlop(mem) => {
                    if !pulse {
                        *mem = !*mem;

                        for dest in &module.destinations {
                            queue.push_back((*mem, module_name, dest));
                        }
                    }
                }
                ModuleType::Conj(mem) => {
                    *mem.get_mut(input).unwrap() = pulse;
                    if module_name == target {
                        if pulse {
                            if *cycles.get(input).unwrap() == 0 {
                                cycles.insert(input, i);
                            }
                        }

                        if cycles.values().all(|n| *n != 0) {
                            break 'outer;
                        }
                    }

                    let resulting_pulse = !mem.values().cloned().all(identity);

                    for dest in &module.destinations {
                        queue.push_back((resulting_pulse, module_name, dest));
                    }
                }
                ModuleType::Basic => {
                    for dest in &module.destinations {
                        queue.push_back((pulse, module_name, dest));
                    }
                }
            }
        }
    }

    let p2 = cycles.values()
        .cloned()
        .reduce(|i, j| i.lcm(&j))
        .unwrap();

    return (low * high, p2);
}
