use regex::Regex;
use std::collections::{HashMap, HashSet};


#[aoc(day7, part1, for_hash)]
pub fn solve_part1(input: &str) -> i32 {
    let nums: Vec<i32> = input
        .split(',')
        .map(|l| {
            l.parse().unwrap()
        })
        .collect();

    let &min_num = nums.iter().min().unwrap();
    let &max_num = nums.iter().max().unwrap();

    println!("nums {:?}", nums);
    println!("min {} max {}", min_num, max_num);

    let mut min_score = 100000000;

    for test in min_num..max_num {
        let mut test_score = 0;

        for num in &nums {
            let dist = (num - test).abs();
            test_score += dist * (dist + 1) / 2;
        }

        println!("test score {}", test_score);
        if test_score < min_score {
            min_score = test_score;
        }
    }

    min_score
}
