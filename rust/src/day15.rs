use crate::AOContext;

fn hash(seq: &str) -> u32 {
    seq.bytes()
        .fold(0, |acc, next| ((acc as u32 + next as u32) * 17 % 256) as u8) as u32
}

enum Command<'a> {
    Set(&'a str, u32),
    Remove(&'a str),
}

impl<'a> Command<'a> {
    fn from(s: &'a str) -> Self {
        match s.split_once("=") {
            Some((label, num)) => Command::Set(label, num.parse::<u32>().unwrap()),
            None => Command::Remove(s.strip_suffix("-").unwrap_or_else(|| panic!("{s}"))),
        }
    }
}

pub fn day15(input: String, ctx: &mut AOContext) {
    //let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    let p1 = input.trim_end().split(",").map(hash).sum::<u32>();

    ctx.submit_part1(p1);

    let mut boxes = vec![Vec::<(&str, u32)>::new(); 256];

    for cmd in input.trim_end().split(",").map(Command::from) {
        match cmd {
            Command::Set(label, num) => {
                let hash = hash(label) as usize;
                match boxes[hash].iter_mut().find(|(s, _n)| *s == label) {
                    Some(pair) => {
                        pair.1 = num;
                    }
                    None => {
                        boxes[hash].push((label, num));
                    }
                }
            }

            Command::Remove(label) => {
                let hash = hash(label) as usize;
                if let Some(i) = boxes[hash].iter().position(|(s, _n)| *s == label) {
                    boxes[hash].remove(i);
                }
            }
        }
    }

    let p2 = boxes.into_iter().enumerate().map(|(i, lenses)| {
        lenses.into_iter().enumerate().map(|(j, lens)| lens.1 * (j + 1) as u32).sum::<u32>() * (i + 1) as u32
    }).sum::<u32>();

    ctx.submit_part2(p2);
}
