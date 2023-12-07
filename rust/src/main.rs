use once_cell::sync::Lazy;
use paste::paste;
use std::collections::HashMap;
mod parsers;

macro_rules! days_decl {
    ($daymap_name:ident : $($ds:literal),*) => {
        $( paste!{
            mod [< day $ds >];
            use [< day $ds >]::*;
        })*

        static $daymap_name: Lazy<HashMap<usize, fn(String)>> = Lazy::new(|| {
            let mut map: std::collections::HashMap<usize, fn(String)> = HashMap::new();
            $( map.insert($ds, paste!{ [< day $ds >] });)*
            map
        });
    }
}

days_decl!(DAYS: 1, 2, 3, 4, 5, 6, 7);

fn main() {
    let day = std::env::args().nth(1);

    let Some(day) = day.and_then(|d| d.parse::<usize>().ok()) else {
        eprintln!("Usage: aoc23rs [day]");
        return;
    };

    let Ok(input) = std::fs::read_to_string(format!("../input/day{day}.txt")) else {
        eprintln!("input file not found! please create it at [REPO ROOT]/input/day{day}.txt");
        return;
    };

    match DAYS.get(&day) {
        None => {
            eprintln!("Day not finished!");
        }

        Some(function) => {
            function(input);
        }
    }
}
