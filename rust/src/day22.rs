use std::cmp::{max, min};

use nom::{
    bytes::complete::tag, multi::separated_list1, sequence::separated_pair, IResult, Parser,
};

use crate::{parsers::int, AOContext};

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
struct Brick {
    coords: [(isize, isize); 3],
}

fn coord(input: &str) -> IResult<&str, (isize, isize, isize)> {
    separated_list1(tag(","), int::<isize>)
        .map(|v| {
            assert_eq!(v.len(), 3);
            (v[0], v[1], v[2])
        })
        .parse(input)
}

fn brick(input: &str) -> IResult<&str, Brick> {
    separated_pair(coord, tag("~"), coord)
        .map(|((x0, y0, z0), (x1, y1, z1))| {
            let x = (min(x0, x1), max(x0, x1));
            let y = (min(y0, y1), max(y0, y1));
            let z = (min(z0, z1), max(z0, z1));
            Brick { coords: [x, y, z] }
        })
        .parse(input)
}

fn bricks_overlap(a: &Brick, b: &Brick) -> bool {
    (0..=1).all(|i| {
        let a = a.coords[i];
        let b = b.coords[i];

        a.0 <= b.0 && a.1 >= b.0
            || a.0 <= b.1 && a.1 >= b.1
            || b.0 <= a.0 && b.1 >= a.0
            || b.0 <= a.1 && b.1 >= a.1
    })
}

#[derive(Default, Clone)]
struct DagEntry {
    parents: Vec<usize>,
    children: Vec<usize>,
}

fn dis_helper(current: usize, dag: &[DagEntry], gone: &mut [bool]) -> usize {
    if dag[current].parents.is_empty() || dag[current].parents.iter().any(|p| !gone[*p]) {
        return 0;
    }

    gone[current] = true;

    dag[current].children
        .iter()
        .map(|&child| dis_helper(child, dag, gone))
        .sum::<usize>() + 1
}

fn disintegrate(start: usize, dag: &[DagEntry]) -> usize {
    let mut gone = vec![false; dag.len()];
    gone[start] = true;

    dag[start].children
        .iter()
        .map(|&child| dis_helper(child, dag, gone.as_mut_slice()))
        .sum()
}

pub fn day22(input: String, ctx: &mut AOContext) {
    let mut bricks = input
        .lines()
        .map(|l| brick(l).unwrap().1)
        .collect::<Vec<_>>();

    ctx.parsing_done();

    bricks.sort_by(|a, b| a.coords[2].1.cmp(&b.coords[2].1));

    let mut arranged = Vec::<Brick>::with_capacity(bricks.len());

    'outer: for mut b in bricks.drain(..) {
        arranged.sort_by(|a, b| a.coords[2].1.cmp(&b.coords[2].1));
        for (i, a) in arranged.iter().enumerate().rev() {
            if bricks_overlap(a, &b) {
                let dist = b.coords[2].0 - a.coords[2].1 - 1;

                b.coords[2].0 -= dist;
                b.coords[2].1 -= dist;

                arranged.insert(i + 1, b);
                continue 'outer;
            }
        }

        b.coords[2].1 -= b.coords[2].0 - 1;
        b.coords[2].0 = 1;
        arranged.push(b);
    }

    let mut dag = vec![DagEntry::default(); arranged.len()];
    let mut bearing = vec![false; arranged.len()];

    for (i, b) in arranged.iter().enumerate() {
        for (j, a) in arranged.iter().enumerate() {
            if i == j {
                continue;
            }
            if bricks_overlap(b, a) && b.coords[2].0 == a.coords[2].1 + 1 {
                dag[j].children.push(i);
                dag[i].parents.push(j);
            }
        }

        if dag[i].parents.len() == 1 {
            bearing[dag[i].parents[0]] = true;
        }
    }

    ctx.submit_part1(bearing.iter().filter(|b| !**b).count());

    let p2 = bearing.iter().enumerate().filter_map(|(i, flag)| flag.then_some(i))
        .map(|i| disintegrate(i, &dag))
        .sum::<usize>();

    ctx.submit_part2(p2);
}
