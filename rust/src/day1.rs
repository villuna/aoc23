use std::cmp::Ordering;

use crate::AOContext;

pub fn day1(input: String, ctx: &mut AOContext) {
    let p1 = input
        .lines()
        .map(|s| {
            (
                s.chars()
                    .find(|c| c.is_ascii_digit())
                    .unwrap()
                    .to_digit(10)
                    .unwrap(),
                s.chars()
                    .rev()
                    .find(|c| c.is_ascii_digit())
                    .unwrap()
                    .to_digit(10)
                    .unwrap(),
            )
        })
        .map(|(a, b)| a * 10 + b)
        .sum::<u32>();

    ctx.submit_part1(p1);

    let vals = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    // Get the first digits
    let p2a = input
        .lines()
        .map(|s| {
            // figure out which pattern comes first in the string, by sorting the patterns by their
            // position in the string
            let mut v = vals.clone();
            v.sort_by(|&(s1, _), &(s2, _)| {
                let x = s.find(s1);
                let y = s.find(s2);

                if let (Some(a), Some(b)) = (x, y) {
                    a.cmp(&b)
                } else if x.is_some() {
                    Ordering::Less
                } else if y.is_some() {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });

            // Then replace the first pattern with its corresponding digit
            s.replace(&v[0].0, &v[0].1.to_string())
        })
        .map(|s| {
            s.chars()
                .find(|c| c.is_ascii_digit())
                .unwrap()
                .to_digit(10)
                .unwrap()
        });

    // Get the last digits
    let p2b = input
        .lines()
        .map(|s| s.chars().rev().collect::<String>())
        .map(|s| {
            let mut v = vals
                .iter()
                .cloned()
                .map(|(ss, v)| (ss.chars().rev().collect::<String>(), v))
                .collect::<Vec<_>>();

            v.sort_by(|(s1, _), (s2, _)| {
                let x = s.find(s1);
                let y = s.find(s2);

                if let (Some(a), Some(b)) = (x, y) {
                    a.cmp(&b)
                } else if x.is_some() {
                    Ordering::Less
                } else if y.is_some() {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });
            s.replace(&v[0].0, &v[0].1.to_string())
        })
        .map(|s| {
            s.chars()
                .find(|c| c.is_ascii_digit())
                .unwrap()
                .to_digit(10)
                .unwrap()
        });

    // Zip them, convert to 2 digit number and get the sum
    let p2_sum = p2a.zip(p2b).map(|(a, b)| a * 10 + b).sum::<u32>();

    ctx.submit_part2(p2_sum);
}
