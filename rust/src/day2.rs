use nom::{IResult, sequence::tuple, bytes::complete::{is_not, tag, is_a}, multi::{many0, separated_list0}, character::complete::digit1, branch::alt};

use crate::AOContext;

const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;

#[derive(Default, Debug)]
struct Hand {
    red: usize,
    green: usize,
    blue: usize,
}

fn hand(input: &str) -> IResult<&str, Hand> {
    let mut hand = Hand::default();

    let (i, cols) = separated_list0(tag(", "), tuple((
        digit1,
        alt((tag(" red"), tag(" blue"), tag(" green")))
    )))(input)?;

    for (num, col) in cols {
        let num = num.parse::<usize>().unwrap();

        match col {
            " red" => { hand.red = num; },
            " blue" => { hand.blue = num; },
            " green" => { hand.green = num; },
            _ => unreachable!(),
        }
    }

    Ok((i, hand))
}

fn parse_games(input: &str) -> IResult<&str, Vec<Vec<Hand>>> {
    let (i, res) = separated_list0(is_a("\r\n"), tuple((
        many0(is_not(":")),
        tag(": "),
        separated_list0(tag("; "), hand),
    )))(input)?;

    let res = res.into_iter().map(|(_, _, r)| r).collect();

    Ok((i, res))
}

pub fn day2(input: String, ctx: &mut AOContext) {
    let input = parse_games(&input).unwrap().1;
    ctx.parsing_done();

    let p1 = input.iter()
        .enumerate()
        .filter(|(_, hands)| hands.iter().all(|hand| hand.red <= MAX_RED && hand.blue <= MAX_BLUE && hand.green <= MAX_GREEN))
        .map(|(i, _)| i + 1)
        .sum::<usize>();

    ctx.submit_part1(p1);

    let p2 = input.iter()
        .map(|hands| (
                hands.iter().map(|hand| hand.red).max().unwrap(),
                hands.iter().map(|hand| hand.green).max().unwrap(),
                hands.iter().map(|hand| hand.blue).max().unwrap(),
            ))
        .map(|(r, g, b)| r * g * b)
        .sum::<usize>();

    ctx.submit_part2(p2);
}
