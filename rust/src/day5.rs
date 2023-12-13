use nom::{
    bytes::complete::tag,
    character::complete::{digit1, one_of},
    multi::{many0, many1, separated_list1},
    sequence::{delimited, preceded, terminated, tuple},
    IResult, Parser,
};
use smallvec::{SmallVec, smallvec};

use crate::AOContext;

#[derive(Debug)]
struct Range {
    source_base: i64,
    dest_base: i64,
    range: i64,
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<i64>,
    ranges: Vec<Vec<Range>>,
}

fn newline(input: &str) -> IResult<&str, ()> {
    many1(one_of("\n\r")).map(|_| ()).parse(input)
}

fn int(input: &str) -> IResult<&str, i64> {
    digit1
        .map(|x: &str| x.parse::<i64>().unwrap())
        .parse(input)
}

fn header(input: &str) -> IResult<&str, Vec<i64>> {
    delimited(tag("seeds: "), separated_list1(tag(" "), int), newline)(input)
}

fn section(input: &str) -> IResult<&str, Vec<Range>> {
    preceded(
        tuple((
            many1(one_of("-abcdefghijklmnopqrstuvwxyz")),
            tag(" map:"),
            newline,
        )),
        many1(terminated(separated_list1(tag(" "), int), newline)),
    )
    .map(|v| {
        v.iter()
            .map(|v| {
                assert_eq!(v.len(), 3);
                Range {
                    source_base: v[1],
                    dest_base: v[0],
                    range: v[2],
                }
            })
            .collect()
    })
    .parse(input)
}

fn almanac(input: &str) -> IResult<&str, Almanac> {
    tuple((header, many0(section)))
        .map(|(seeds, ranges)| Almanac { seeds, ranges })
        .parse(input)
}

pub fn day5(input: String, ctx: &mut AOContext) {
    let almanac = almanac(&input).unwrap().1;
    ctx.parsing_done();
    ctx.submit_part1(part1(&almanac));
    ctx.submit_part2(part2(&almanac));
}

fn part1(almanac: &Almanac) -> i64 {
    let mut seeds = almanac.seeds.clone();

    for ranges in almanac.ranges.iter() {
        for seed in seeds.iter_mut() {
            if let Some(new_seed) = ranges.iter().find_map(
                |&Range {
                     source_base,
                     dest_base,
                     range,
                 }| {
                    if *seed >= source_base && *seed < source_base + range {
                        Some(dest_base - source_base + *seed)
                    } else {
                        None
                    }
                },
            ) {
                *seed = new_seed;
            }
        }
    }

    *seeds.iter().min().unwrap()
}

// Takes an interval and a range and splits it up into a list of unmapped intervals and a list of
// mapped intervals, respectively.
// I've optimised it, since there will only be at most 2 unmapped and one mapped, im using a
// smallvec and option respectively
fn intersect_intervals((start, end): (i64, i64), range: &Range) -> (SmallVec<[(i64, i64); 2]>, Option<(i64, i64)>) {
    let &Range { 
        source_base,
        dest_base,
        range
    } = range;

    if start >= source_base && start < source_base + range {
        if end <= source_base + range {
            (
                smallvec![],
                Some((start + dest_base - source_base, end + dest_base - source_base))
            )
        } else {
            (
                smallvec![(source_base + range, end)],
                Some((start + dest_base - source_base, dest_base + range))
            )
        }
    } else if start < source_base && end > source_base {
        if end <= source_base + range {
            (
                smallvec![(start, source_base)],
                Some((dest_base, end - source_base + dest_base))
            )
        } else {
            (
                smallvec![(start, source_base), (source_base + range, end)],
                Some((dest_base, dest_base + range))
            )
        }
    } else {
        (
            smallvec![(start, end)],
            None,
        )
    }
}

fn part2(almanac: &Almanac) -> i64 {
    let mut seeds = almanac
        .seeds
        .chunks_exact(2)
        .map(|range| (range[0], range[0] + range[1]))
        .collect::<Vec<_>>();

    for ranges in almanac.ranges.iter() {
        let mut next_seeds: Vec<(i64, i64)> = Vec::with_capacity(161);

        for &seed in seeds.iter() {
            let mut current_seed = vec![seed];
            let mut mapped_sections = vec![];

            for range in ranges.iter() {
                let mut unmapped_sections = vec![];

                for interval in current_seed.into_iter() {
                    let (new_unmapped, new_mapped) = intersect_intervals(interval, range);
                    mapped_sections.extend(new_mapped);
                    unmapped_sections.extend(new_unmapped);
                }

                current_seed = unmapped_sections;
            }

            next_seeds.extend(current_seed.into_iter());
            next_seeds.extend(mapped_sections.into_iter());
        }

        seeds = next_seeds;
    }

    *seeds.iter().map(|(a, _)| a).min().unwrap()
}
