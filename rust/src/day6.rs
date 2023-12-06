use nom::{sequence::{separated_pair, preceded}, bytes::complete::tag};
use crate::parsers::{int, newline, space_separated_ints, strip};

pub fn day6(mut input: String) {
    let (times, distances) = separated_pair(
        preceded(strip(tag("Time:")), space_separated_ints::<i64>),
        newline,
        preceded(strip(tag("Distance:")), space_separated_ints::<i64>),
    )(&input).unwrap().1;

    let p1 = times.iter()
        .zip(distances.iter())
        .map(|(&time, &dist)| {
            (0..=time).map(|i| i * (time - i))
                .filter(|d| *d > dist)
                .count()
        }).product::<usize>();

    println!("part 1: {p1}");

    // Get rid of all the spaces for the second part
    input.retain(|c| c != ' ');

    let (time, distance) = separated_pair(
        preceded(tag("Time:"), int::<i128>),
        newline,
        preceded(tag("Distance:"), int::<i128>),
    )(&input).unwrap().1;

    let p2 = (0..=time).map(|i| i * (time - i))
        .filter(|d| *d > distance)
        .count();

    println!("part 2: {p2}");
}
