use crate::AOContext;

fn parse_input(input: &str, expansion: u128) -> Vec<(u128, u128)> {
    let mut empty_cols = (0..input.lines().next().unwrap().len()).collect::<Vec<_>>();

    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            if c == '#' {
                empty_cols.retain(|j| *j != i);
            }
        }
    }

    let mut res = Vec::new();
    let mut y = 0;
    for line in input.lines() {
        let mut encountered_any = false;
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                encountered_any = true;
                res.push((x as u128 + empty_cols.iter().filter(|i| **i < x).count() as u128 * (expansion - 1), y));
            }
        }

        if encountered_any {
            y += 1;
        } else {
            y += expansion;
        }
    }

    res
}

fn solve(input: &str, expansion: u128) -> u128 {
    let galaxies = parse_input(&input, expansion);
    let mut total = 0;

    for &g in &galaxies {
        for &h in &galaxies {
            if g != h {
                total += g.0.abs_diff(h.0) + g.1.abs_diff(h.1);
            }
        }
    }

    total / 2
}

pub fn day11(input: String, ctx: &mut AOContext) {
    ctx.submit_part1(solve(&input, 2));
    ctx.submit_part2(solve(&input, 1000000));
}
