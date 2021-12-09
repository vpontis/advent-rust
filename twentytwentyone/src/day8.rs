use std::collections::{HashMap};


#[aoc(day8, part1, for_hash)]
pub fn solve_part1(input: &str) -> i32 {
    let mut answer = 0;

    for line in input.lines() {
        // println!("line {}", line);
        let line_parts: Vec<&str> = line.split("|").collect();
        let words: Vec<&str> = line_parts[1].trim().split(" ").collect();

        for word in words {
            // println!("word {}", word);
            let l = word.len();
            if l == 2 || l == 4 || l == 3 || l == 7 {
                answer += 1;
            }
        }
    }

    answer
}


#[aoc(day8, part2, for_hash)]
pub fn solve_part2(input: &str) -> i32 {
    let mut answer = 0;

    let mut count_to_num = HashMap::new();

    count_to_num.insert(sort("868497"), 0);
    count_to_num.insert(sort("88747"), 2);
    count_to_num.insert(sort("88797"), 3);
    count_to_num.insert(sort("86797"), 5);
    count_to_num.insert(sort("867497"), 6);
    count_to_num.insert(sort("868797"), 9);


    let mut result = 0;
    for line in input.lines() {
        // println!("line {}", line);
        let line_parts: Vec<&str> = line.split("|").collect();
        let input_words: Vec<&str> = line_parts[0].trim().split(" ").collect();
        let output_words: Vec<&str> = line_parts[1].trim().split(" ").collect();

        let mut char_to_count: HashMap<char, u8> = HashMap::new();

        for word in input_words {
            for char in word.chars() {
                let count = char_to_count.entry(char).or_insert(0);
                *count += 1;
            }
        }

        let mut word_num = 0;

        for (i, word) in output_words.iter().enumerate() {
            let mut num = 0;
            let l = word.len();
            println!("word {} - {}", word, l);
            if l == 2 {
                num = 1;
            } else if l == 4 {
                num = 4;
            } else if l == 3 {
                num = 7;
            } else if l == 7 {
                num = 8;
            } else {
                let word_as_chars: Vec<char> = word
                    .trim()
                    .chars()
                    .map(|c| {
                        char_to_count[&c].to_string().chars().next().unwrap()
                    })
                    .collect();

                let word_count_string: String = word_as_chars.iter().collect();
                let key = sort(&word_count_string);
                println!("key {}", key);

                if let Some(n) = count_to_num.get(&key) {
                    num = *n;
                } else {
                    println!("OH NO {:?} - {} - {:?}", count_to_num, sort(&word_count_string), word_as_chars);
                    panic!("MARISSSSSSSSSSSSA");
                }
            }

            println!("adding {} to {}", num, word_num);
            word_num *= 10;
            word_num += num;
        }

        result += word_num;
    }

    result
}


fn sort(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_by(|a, b| b.cmp(a));

    let s = String::from_iter(chars);
    s
}
