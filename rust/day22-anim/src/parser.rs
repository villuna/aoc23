use std::{cmp::{max, min}, str::FromStr};

use nom::{
    bytes::complete::tag, multi::separated_list1, sequence::separated_pair, IResult, Parser, combinator::map_res, character::complete::digit1,
};

pub fn int<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(digit1, |s: &str| s.parse::<T>())(input)
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Brick {
    pub coords: [(isize, isize); 3],
}

fn coord(input: &str) -> IResult<&str, (isize, isize, isize)> {
    separated_list1(tag(","), int::<isize>)
        .map(|v| {
            assert_eq!(v.len(), 3);
            (v[0], v[1], v[2])
        })
        .parse(input)
}

pub fn brick(input: &str) -> IResult<&str, Brick> {
    separated_pair(coord, tag("~"), coord)
        .map(|((x0, y0, z0), (x1, y1, z1))| {
            let x = (min(x0, x1), max(x0, x1));
            let y = (min(y0, y1), max(y0, y1));
            let z = (min(z0, z1), max(z0, z1));
            Brick { coords: [x, y, z] }
        })
        .parse(input)
}