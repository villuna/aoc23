use crate::AOContext;

fn line(input: &str) -> (Vec<u32>, Vec<u32>) {
    let x = input.split(":").nth(1).unwrap();
    let mut sides = x.split("|");

    let winning = sides.next().unwrap();
    let have = sides.next().unwrap();
    let winning = winning.split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect::<Vec<_>>();
    let have = have.split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect::<Vec<_>>();

    (winning, have)
}

fn winning_numbers_count((winning, have): &(Vec<u32>, Vec<u32>)) -> usize {
    have.iter().filter(|n| winning.contains(n)).count()
}

pub fn day4(input: String, ctx: &mut AOContext) {
    let cards = input.lines()
        .map(line)
        .collect::<Vec<_>>();
    ctx.parsing_done();

    let p1 = cards.iter().map(winning_numbers_count)
        .filter(|n| *n > 0)
        .map(|n| 2u32.pow((n - 1) as u32))
        .sum::<u32>();

    ctx.submit_part1(p1);

    let mut card_count = vec![1; cards.len()];

    for i in 0..cards.len() {
        let card = &cards[i];
        let count = card_count[i];

        let wins = winning_numbers_count(card);

        for j in 0..wins {
            if i + j < cards.len() {
                card_count[i + j + 1] += count;
            }
        }
    }

    let p2 = card_count.iter().sum::<usize>();

    ctx.submit_part2(p2);
}
