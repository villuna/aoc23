mod day1;
use std::collections::HashMap;

use day1::*;

fn main() {
    let mut day_map: HashMap<usize, fn(String)> = HashMap::new();
    day_map.insert(1, day1);

    let day = std::env::args()
        .nth(1);

    match day.and_then(|d| d.parse::<usize>().ok()) {
        None => {
            eprintln!("Usage: aoc23rs [day]");
        }

        Some(day) => {
            let input = std::fs::read_to_string(format!("../input/day{day}.txt")).unwrap();

            match day_map.get(&day) {
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
