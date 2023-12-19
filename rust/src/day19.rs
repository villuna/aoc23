use std::collections::HashMap;

use nom::sequence::preceded;
use nom::Parser;
use nom::{
    bytes::complete::{is_a, is_not, tag},
    character::complete::one_of,
    combinator::opt,
    multi::{many0, separated_list1},
    sequence::{delimited, pair, separated_pair, tuple},
    IResult,
};

use crate::{
    parsers::{int, newline},
    AOContext,
};

#[derive(Debug, Copy, Clone)]
enum Destination<'a> {
    Accepted,
    Rejected,
    Transfer(&'a str),
}

impl<'a> Destination<'a> {
    fn from(dest: &'a str) -> Self {
        match dest {
            "A" => Destination::Accepted,
            "R" => Destination::Rejected,
            s => Destination::Transfer(s),
        }
    }

    fn to_str(&self) -> &'a str {
        match self {
            Destination::Accepted => "A",
            Destination::Rejected => "R",
            Destination::Transfer(dest) => dest,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Rule<'a> {
    Compare {
        id: usize,
        lt: bool,
        num: u32,
        dest: Destination<'a>,
    },
    Fallthrough(Destination<'a>),
}

fn name(input: &str) -> IResult<&str, &str> {
    is_a("abcdefghijklmnopqrstuvwxyzAR")(input)
}

fn rules(input: &str) -> IResult<&str, Vec<Rule>> {
    separated_list1(
        tag(","),
        tuple((
            name,
            opt(separated_pair(
                pair(one_of("><").map(|c| c == '<'), int::<u32>),
                tag(":"),
                name,
            )),
        ))
        .map(|(name, rule)| match rule {
            Some(((lt, num), dest)) => Rule::Compare {
                id: "xmas"
                    .chars()
                    .position(|c| c == name.chars().next().unwrap())
                    .unwrap(),
                lt,
                num,
                dest: Destination::from(dest),
            },

            None => Rule::Fallthrough(Destination::from(name)),
        }),
    )(input)
}

fn workflow(input: &str) -> IResult<&str, (&str, Vec<Rule>)> {
    tuple((is_not("{"), delimited(tag("{"), rules, tag("}")))).parse(input)
}

fn part(input: &str) -> IResult<&str, [u32; 4]> {
    delimited(
        tag("{"),
        separated_list1(
            tag(","),
            preceded(pair(one_of("xmas"), tag("=")), int::<u32>),
        ),
        tag("}"),
    )
    .map(|s| {
        assert_eq!(s.len(), 4);
        [s[0], s[1], s[2], s[3]]
    })
    .parse(input)
}

fn part1<'a>(rules: &HashMap<&'a str, Vec<Rule<'a>>>, parts: &[[u32; 4]]) -> u32 {
    parts
        .iter()
        .filter(|part| {
            let mut wf_name = "in";

            loop {
                let wf = rules.get(wf_name).unwrap();
                let mut d = None;

                for rule in wf {
                    match rule {
                        Rule::Compare { id, lt, num, dest } => {
                            let comparison: Box<dyn Fn(u32) -> bool> = if *lt {
                                Box::new(move |x| x < *num)
                            } else {
                                Box::new(move |x| x > *num)
                            };

                            if comparison(part[*id]) {
                                d = Some(*dest);
                                break;
                            }
                        }
                        Rule::Fallthrough(dest) => {
                            d = Some(*dest);
                            break;
                        }
                    }
                }

                match d.unwrap() {
                    Destination::Accepted => {
                        return true;
                    }
                    Destination::Rejected => {
                        return false;
                    }
                    Destination::Transfer(dest) => wf_name = dest,
                }
            }
        })
        .map(|part| part.iter().sum::<u32>())
        .sum::<u32>()
}

fn count_possibilities<'a>(rules: &HashMap<&'a str, Vec<Rule<'a>>>, mut range: [(u32, u32); 4], wf: &'a str) -> usize {
    if wf == "A" {
        return range.iter()
            .map(|(from, to)| (to - from + 1) as usize)
            .product::<usize>()
    } else if wf == "R" {
        return 0;
    }

    let wf = rules.get(wf).unwrap();
    let mut count = 0;

    for rule in wf {
        match rule {
            Rule::Compare { id, lt, num, dest } => {
                if *lt {
                    if range[*id].1 < *num {
                        count += count_possibilities(rules, range, dest.to_str());
                        break;
                    } else if range[*id].0 < *num {
                        let mut transferred_range = range;
                        transferred_range[*id].1 = *num - 1;
                        count += count_possibilities(rules, transferred_range, dest.to_str());
                        range[*id].0 = *num;
                    }
                } else {
                    if range[*id].0 > *num {
                        count += count_possibilities(rules, range, dest.to_str());
                        break;
                    } else if range[*id].1 > *num {
                        let mut transferred_range = range;
                        transferred_range[*id].0 = *num + 1;
                        count += count_possibilities(rules, transferred_range, dest.to_str());
                        range[*id].1 = *num;
                    }
                }
            },
            Rule::Fallthrough(dest) => count += count_possibilities(rules, range, dest.to_str()),
        }
    }

    count
}

fn part2<'a>(rules: &HashMap<&'a str, Vec<Rule<'a>>>) -> usize {
    count_possibilities(rules, [(1, 4000); 4], "in")
}

pub fn day19(input: String, ctx: &mut AOContext) {
    let (rules, parts) = separated_pair(
        separated_list1(newline, workflow),
        many0(newline),
        separated_list1(newline, part),
    )(&input)
    .unwrap()
    .1;

    let rules: HashMap<&str, Vec<Rule<'_>>> = HashMap::from_iter(rules.into_iter());

    ctx.parsing_done();

    ctx.submit_part1(part1(&rules, &parts));
    ctx.submit_part2(part2(&rules));
}
