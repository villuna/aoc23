use crate::AOContext;

type NumberMap = Vec<(u32, Vec<(usize, usize)>)>;

fn number_map(input: &str) -> NumberMap {
    let mut res = Vec::new();
    for (i, line) in input.lines().enumerate() {
        let mut iter = line.chars().enumerate().peekable();

        'outer: loop {
            loop {
                match iter.peek() {
                    None => break 'outer,
                    Some(c) => {
                        if c.1.is_ascii_digit() {
                            break;
                        } else {
                            iter.next();
                        }
                    }
                }
            }

            let mut number_string = String::new();
            let mut map = Vec::new();

            while let Some(&(j, c)) = iter.peek() {
                if c.is_ascii_digit() {
                    number_string.push(c);
                    map.push((i, j));
                    iter.next();
                } else {
                    break;
                }
            }

            res.push((number_string.parse().unwrap(), map));
        }
    }

    res
}

fn char_pos_map(input: &str) -> Vec<(char, (usize, usize))> {
    input.lines().enumerate()
        .map(|(i, line)| {
            line.chars().enumerate()
                .filter(|&(_j, c)| !c.is_ascii_digit() && c != '.')
                .map(move |(j, c)|
                    (c, (i, j))
                )
        }).flatten()
        .collect()
}

fn are_adjacent(&(ai, aj): &(usize, usize), &(bi, bj): &(usize, usize)) -> bool {
    ai.abs_diff(bi) == 1 && aj.abs_diff(bj) == 1
}

pub fn day3(input: String, ctx: &mut AOContext) {
    let numbers = number_map(&input);
    let char_positions = char_pos_map(&input);
    
    ctx.parsing_done();

    let p1 = numbers.iter()
        .filter(|(_, coords)| {
            coords.iter().any(|num_coord|
                char_positions.iter().any(|(_, char_coord)| are_adjacent(num_coord, char_coord))
            )
        }).map(|(num, _)| num)
        .sum::<u32>();

    ctx.submit_part1(p1);

    let p2 = char_positions.iter()
        .filter(|(c, _)| *c == '*')
        .filter_map(|(_, char_coord)| {
            let nums = numbers.iter()
                .filter(|(_, num_coords)| 
                    num_coords.iter().any(|num_coord| are_adjacent(char_coord, num_coord))
                ).collect::<Vec<_>>();

            if nums.len() != 2 {
                None
            } else {
                Some(nums[0].0 * nums[1].0)
            }
        }).sum::<u32>();

    ctx.submit_part2(p2);
}
