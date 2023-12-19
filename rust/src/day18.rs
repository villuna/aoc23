use std::num::ParseIntError;

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::one_of,
    combinator::{map_res, recognize},
    multi::many_m_n,
    sequence::{delimited, pair, preceded},
    Parser,
};

use crate::{
    parsers::{int, strip},
    utils::{Coord, Dir},
    AOContext,
};

fn solve(commands: impl Iterator<Item = (Dir, isize)>) -> isize {
    let mut coord = Coord(0, 0);
    let mut area = 0;

    for (dir, dist) in commands {
        let new_coord = coord + dir.cincrement() * dist;

        area += (new_coord.1 + coord.1) * (coord.0 - new_coord.0);
        area += dist;
        coord = new_coord;
    }

    area.abs() / 2 + 1
}

pub fn day18(input: String, ctx: &mut AOContext) {
    let p2_command = delimited(
        tag("("),
        map_res(
            preceded(
                tag("#"),
                pair(
                    recognize(many_m_n(5, 5, one_of("0123456789abcdef"))),
                    one_of("0123"),
                ),
            ),
            |(s, d)| {
                let dist = isize::from_str_radix(s, 16)?;
                let dir = match d {
                    '0' => Dir::Right,
                    '1' => Dir::Down,
                    '2' => Dir::Left,
                    '3' => Dir::Up,
                    _ => unreachable!(),
                };

                Ok::<_, ParseIntError>((dir, dist))
            }
        ),
        tag(")"),
    );

    let mut command = pair(
        pair(
            strip(one_of("UDLR")).map(Dir::from_char),
            strip(int::<isize>),
        ),
        p2_command
    );

    let commands = input
        .lines()
        .map(|line| command(line).unwrap().1)
        .collect_vec();
    ctx.parsing_done();

    let p1_commands = commands.iter()
        .cloned()
        .map(|(p1, _p2)| p1);

    ctx.submit_part1(solve(p1_commands));

    let p2_commands = commands.iter()
        .cloned()
        .map(|(_p1, p2)| p2);

    ctx.submit_part2(solve(p2_commands));
}
