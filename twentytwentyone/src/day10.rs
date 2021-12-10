use std::collections::{HashMap, HashSet};

#[aoc(day10, part1, for_hash)]
pub fn solve_part1(input: &str) -> i32 {
    let mut char_to_score = HashMap::new();
    char_to_score.insert(')', 3);
    char_to_score.insert(']', 57);
    char_to_score.insert('}', 1197);
    char_to_score.insert('>', 25137);

    let mut bads = [0, 0, 0, 0];

    for line in input.lines() {
        println!("line {}", line);
        let mut stack: Vec<char> = Vec::new();

        for char in line.chars() {
            match char {
                '(' => stack.push(char),
                '[' => stack.push(char),
                '<' => stack.push(char),
                '{' => stack.push(char),
                ')' => {
                    let char = stack.pop().unwrap();
                    if char != '(' {
                        bads[0] += 1;
                        break;
                    }
                }
                ']' => {
                    let char = stack.pop().unwrap();
                    if char != '[' {
                        bads[1] += 1;
                        break;
                    }
                }
                '}' => {
                    let char = stack.pop().unwrap();
                    if char != '{' {
                        bads[2] += 1;
                        break;
                    }
                }
                '>' => {
                    let char = stack.pop().unwrap();
                    if char != '<' {
                        bads[3] += 1;
                        break;
                    }
                }
                _ => panic!("haaaaaaaa"),
            }
        }
    }

    println!("bads {:?}", bads);
    let mut answer = 0;
    bads[0] * 3 + bads[1] * 57 + bads[2] * 1197 + bads[3] * 25137
}

#[aoc(day10, part2, for_hash)]
pub fn solve_part2(input: &str) -> i64 {
    let mut scores = Vec::new();

    for line in input.lines() {
        println!("line {}", line);
        let mut stack: Vec<char> = Vec::new();

        let mut bad = false;

        for char in line.chars() {
            match char {
                '(' => stack.push(char),
                '[' => stack.push(char),
                '<' => stack.push(char),
                '{' => stack.push(char),
                ')' => {
                    let char = stack.pop().unwrap();
                    if char != '(' {
                        bad = true;
                        break;
                    }
                }
                ']' => {
                    let char = stack.pop().unwrap();
                    if char != '[' {
                        bad = true;
                        break;
                    }
                }
                '}' => {
                    let char = stack.pop().unwrap();
                    if char != '{' {
                        bad = true;
                        break;
                    }
                }
                '>' => {
                    let char = stack.pop().unwrap();
                    if char != '<' {
                        bad = true;
                        break;
                    }
                }
                _ => panic!("haaaaaaaa"),
            }
        }

        if bad {
            continue;
        }


        let mut score: i64 = 0;
        println!("stack {:?}", stack);
        while stack.is_empty() == false {
            let char = stack.pop().unwrap();
            score *= 5;
            if char == '[' {
                score += 2;
            }
            if char == '(' {
                score += 1;
            }
            if char == '<' {
                score += 4;
            }
            if char == '{' {
                score += 3;
            }
        }
        println!("score {}", score);

        scores.push(score);

    }

    println!("scores {:?}", scores);

    scores.sort();
    println!("sorted {:?}", scores);
    let score_len = scores.len();
    scores[score_len / 2]
}
