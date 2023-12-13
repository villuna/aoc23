use std::collections::HashMap;

use crate::AOContext;

#[derive(PartialEq, Eq, Ord, PartialOrd, Debug, Clone, Copy)]
enum Ranking {
    High,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum Part { Part1, Part2 }

#[derive(PartialEq, Eq, Ord, Clone, Debug)]
struct Hand(String, Part);

impl Hand {
    fn ranking(&self) -> (Ranking, Hand) {
        if self.1 == Part::Part1 {
            (self.raw_ranking(), self.clone())
        } else {
            let cards = ['J', '2','3','4','5','6','7','8','9','T','Q','K','A'];
            let mut possible_hands = vec![Hand(self.0.clone(), Part::Part1)];

            for (i, _) in self.0.chars().enumerate().filter(|(_, c)| *c == 'J') {
                possible_hands = possible_hands.into_iter().flat_map(|hand| {
                    let mut v = Vec::new();
                    for c in &cards {
                        let mut hand_str = String::new();
                        hand_str.push_str(&hand.0[0..i]);
                        hand_str.push(*c);
                        hand_str.push_str(&hand.0[i+1..]);
                        v.push(Hand(hand_str, Part::Part1));
                    }
                    v
                }).collect::<Vec<_>>();
            }

            let possible_hands = possible_hands.into_iter().map(|hand| hand.ranking())
                .collect::<Vec<_>>();
            possible_hands.iter().max().unwrap().clone()
        }
    }

    fn raw_ranking(&self) -> Ranking {
        let mut chars = HashMap::new();
        for c in self.0.chars() {
            *chars.entry(c).or_insert(0) += 1;
        }

        if chars.len() == 1 {
            Ranking::FiveKind
        } else {
            match chars.values().max().unwrap() {
                4 => Ranking::FourKind,    
                3 => {
                    if chars.values().find(|x| **x == 2).is_some() {
                        Ranking::FullHouse
                    } else {
                        Ranking::ThreeKind
                    }
                },
                2 => {
                    if chars.values().filter(|x| **x == 2).count() == 2 {
                        Ranking::TwoPair
                    } else {
                        Ranking::OnePair
                    }
                },

                1 => {
                    Ranking::High
                } 
                _ => unreachable!(),
            }
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let order = if self.1 == Part::Part1 {
            ['2','3','4','5','6','7','8','9','T','J','Q','K','A']
        } else {
            ['J', '2','3','4','5','6','7','8','9','T','Q','K','A']
        };

        let s = self.0.chars()
            .map(|c| order.iter().position(|x| *x == c))
            .collect::<Vec<_>>();

        let o = other.0.chars()
            .map(|c| order.iter().position(|x| *x == c))
            .collect::<Vec<_>>();

        s.partial_cmp(&o)
    }
}

fn solve_part(input: &[(&str, i64)], part: Part) -> i64 {
    let mut input = input.iter()
        .map(|(hand, num)| (Hand(hand.to_string(), part), *num))
        .collect::<Vec<_>>();

    input.sort_by_key(|(hand, _)| hand.ranking());

    input.into_iter()
        .enumerate()
        .map(|(i, (_, num))| (i as i64 + 1) * num)
        .sum::<i64>()
}

pub fn day7(input: String, ctx: &mut AOContext) {
    let input = input.lines()
        .map(|line| {
            let mut splits = line.split(" ");
            (splits.next().unwrap(), splits.next().unwrap().parse::<i64>().unwrap())
        }).collect::<Vec<_>>();
    ctx.parsing_done();

    ctx.submit_part1(solve_part(&input, Part::Part1));
    ctx.submit_part2(solve_part(&input, Part::Part2));
}
