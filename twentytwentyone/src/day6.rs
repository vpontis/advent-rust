use regex::Regex;
use std::collections::{HashMap, HashSet};


#[aoc(day6, part1, for_hash)]
pub fn solve_part1(input: &str) -> i64 {
    // TODO: why can't this go outside a function
    let iterations = 256;

    let nums: Vec<i64> = input
        .split(',')
        .map(|l| {
            l.parse().unwrap()
        })
        .collect();

    let mut num_to_count: HashMap<i64, i64> = HashMap::new();
    for num in nums {
        let count = num_to_count.entry(num).or_insert(0);
        *count += 1;
    }

    for i in 1..(iterations + 1) {
        let mut new: HashMap<i64, i64> = HashMap::new();

        for x in 1..(8 + 1) {
            let xcount = num_to_count.get(&x);
            if let Some(&xcount) = xcount {
                println!("Updating ha {} {}", x, xcount);
                new.insert(x - 1, xcount);
            }
        }

        let zero_count = num_to_count.get(&0);
        if let Some(&zero_count) = zero_count {
            new.insert(8, zero_count);

            let six_count = new.entry(6).or_insert(0);
            *six_count += zero_count;
        }

        println!("day {} - values {:?}", i, new);
        num_to_count = new.clone();
    }

    println!("ha nums {:?}", num_to_count);

    let mut num_fish = 0;
    for (_, val) in num_to_count {
        num_fish += val;
    }


    num_fish
}
