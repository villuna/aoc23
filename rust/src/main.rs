use std::collections::HashMap;
use once_cell::sync::Lazy;
use paste::paste;

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

days_decl!(DAYS: 1);

fn main() {
    let day = std::env::args()
        .nth(1);

    match day.and_then(|d| d.parse::<usize>().ok()) {
        None => {
            eprintln!("Usage: aoc23rs [day]");
        }

        Some(day) => {
            let input = std::fs::read_to_string(format!("../input/day{day}.txt")).unwrap();

            match DAYS.get(&day) {
                None => {
                    eprintln!("Day not finished!");
                },

                Some(function) => {
                    function(input);
                }
            }
        }
    }
}
