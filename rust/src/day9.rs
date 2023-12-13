use crate::{parsers::kait_ints, AOContext};

pub fn day9(input: String, ctx: &mut AOContext) {
    let nums: Vec<Vec<i64>> = input.lines()
        .map(kait_ints)
        .collect::<Vec<_>>();

    ctx.parsing_done();

    let (p1, p2) = nums.iter().map(|vec| {
        let mut difference_stack = vec![vec.clone()];
        
        loop {
            let difference = difference_stack.last().unwrap().windows(2)
                .map(|list| list[1] - list[0])
                .collect::<Vec<_>>();

            if difference.iter().all(|x| *x == 0) {
                break;
            } else {
                difference_stack.push(difference);
            }
        }

        let mut p1 = 0;
        let mut p2 = 0;

        while let Some(diffs) = difference_stack.pop() {
            p1 += diffs.last().unwrap(); 
            p2 = diffs.first().unwrap() - p2;
        }

        (p1, p2)
    }).reduce(|(a, b), (c, d)| (a + c, b + d)).unwrap();

    ctx.submit_both(p1, p2);
}
