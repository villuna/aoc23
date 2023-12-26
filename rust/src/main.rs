use once_cell::sync::Lazy;
use paste::paste;
use std::{collections::HashMap, time::Instant, fmt::Display};
use clap::Parser;
mod parsers;
mod utils;

macro_rules! days_decl {
    ($daymap_name:ident : $($ds:literal),*) => {
        $( paste!{
            mod [< day $ds >];
            use [< day $ds >]::*;
        })*

        static $daymap_name: Lazy<HashMap<usize, fn(String, &mut AOContext)>> = Lazy::new(|| {
            let mut map: std::collections::HashMap<usize, fn(String, &mut AOContext)> = HashMap::new();
            $( map.insert($ds, paste!{ [< day $ds >] });)*
            map
        });
    }
}

// merry christmas!
days_decl!(DAYS: 1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 13, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25);

#[derive(Parser)]
struct Args {
    /// the day to solve
    day: usize,
    /// time solutions and print the benchmark with the result
    #[arg(short)]
    time: bool,
}

struct AOContext {
    now: Instant,
    parsing_time: Option<f64>,
    p1_time: f64,
    time: bool,
}

impl AOContext {
    fn new(time: bool) -> AOContext {
        AOContext { now: Instant::now(), parsing_time: None, p1_time: 0.0, time }
    }

    pub fn parsing_done(&mut self) {
        if self.time {
            self.parsing_time = Some(self.now.elapsed().as_secs_f64() * 1000.0);
            self.now = Instant::now();
        }
    }

    pub fn submit_part1<T: Display>(&mut self, result: T) {
        println!("part 1: {result}");

        if self.time {
            self.p1_time = self.now.elapsed().as_secs_f64() * 1000.0;
            self.now = Instant::now();
        }
    }

    pub fn submit_part2<T: Display>(&mut self, result: T) {
        println!("part 2: {result}");

        if self.time {
            println!("");
            let p2_time = self.now.elapsed().as_secs_f64() * 1000.0;
            if let Some(parsing) = self.parsing_time {
                println!("parsing took {:.2}ms", parsing);
            }
            println!("part 1 took {:.2}ms\npart 2 took {:.2}ms", self.p1_time, p2_time);
        }
    }

    pub fn submit_both<P1: Display, P2: Display>(&mut self, p1: P1, p2: P2) {
        println!("part 1: {p1}\npart 2: {p2}");

        if self.time {
            println!("");
            let time = self.now.elapsed().as_secs_f64() * 1000.0;
            if let Some(parsing) = self.parsing_time {
                println!("parsing took {:.2}ms", parsing);
            }
            println!("solving took {:.2}ms", time);
        }
    }
}

fn main() {
    let args = Args::parse();
    let day = args.day;

    match DAYS.get(&day) {
        None => {
            eprintln!("Day invalid or not completed!");
        }

        Some(function) => {
            let Ok(input) = std::fs::read_to_string(format!("../input/day{day}.txt")) else {
                eprintln!("input file not found! please create it at [REPO ROOT]/input/day{day}.txt");
                return;
            };

            let mut ctx = AOContext::new(args.time);
            function(input, &mut ctx);
        }
    }
}
