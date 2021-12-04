use std::collections::{HashMap, HashSet};
use regex::Regex;

#[derive(Hash, Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl PartialEq<Self> for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

struct Board {
    name: i32,
    matrix: [[i32; 5]; 5],
    num_to_point: HashMap<i32, Point>,
    seen_points: HashSet<Point>,
}

impl Board {
    fn add_num(&mut self, num: i32) {
        match self.num_to_point.get(&num) {
            Some(&p) => {
                self.seen_points.insert(p);
            }
            _ => {}
        }
    }

    fn is_winner(&self) -> bool {
        // check if it is a column winner
        for x in 0..5 {
            for y in 0..5 {
                let point = Point { x, y };

                if self.seen_points.contains(&point) == false {
                    break;
                }

                if y == 4 {
                    return true;
                }
            }
        }

        // check if it is a row winner
        for y in 0..5 {
            for x in 0..5 {
                let point = Point { x, y };

                if self.seen_points.contains(&point) == false {
                    break;
                }

                if x == 4 {
                    return true;
                }
            }
        }

        false
    }

    fn winning_score(&self, multiple: i32) -> i32 {
        // check if it is a row winner
        let mut score = 0;

        for y in 0..5 {
            for x in 0..5 {
                let point = Point { x, y };
                if self.seen_points.contains(&point) == false {
                    score += self.matrix[x as usize][y as usize];
                }
            }
        }

        println!("Before multiple {}", score);
        score * multiple
    }
}


#[aoc(day4, part1, for_hash)]
pub fn solve_part1(input: &str) -> i32 {
    let space_re = Regex::new(r"\s+").unwrap();
    let chunks = input.split("\n\n");

    let mut chunks_iter = chunks.into_iter();

    let nums: Vec<i32> = chunks_iter
        .next()
        .unwrap()
        .split(",")
        .map(|l| {
            l.parse().unwrap()
        })
        .collect();

    let mut boards: Vec<Board> = Vec::new();

    for (board_idx, chunk) in chunks_iter.enumerate() {
        let mut matrix = [[0; 5]; 5];

        let mut num_to_point = HashMap::new();

        for (y, row) in chunk.split("\n").into_iter().enumerate() {
            let col_nums: Vec<i32> = space_re
                .split(row.trim())
                .map(|c| { c.parse().unwrap() })
                .collect();

            for (x, num) in col_nums.into_iter().enumerate() {
                let point = Point { x: x as i32, y: y as i32 };

                matrix[x][y] = num;
                num_to_point.insert(num, point);
            }
        }

        let board = Board {
            name: board_idx as i32,
            matrix,
            num_to_point,
            seen_points: HashSet::new(),
        };

        boards.push(board);
    }

    println!("Boards length {}", boards.len());

    for num in nums {
        for board_idx in 0..boards.len() {
            boards[board_idx].add_num(num);

            if boards[board_idx].is_winner() {
                println!("WINNNNNNNNNNNNNNNER {}", board_idx);
                let output = boards[board_idx].winning_score(num);
                return output;
            }
        }
    }

    10
}

#[aoc(day4, part2, for_hash)]
pub fn solve_part2(input: &str) -> i32 {
    let space_re = Regex::new(r"\s+").unwrap();
    let chunks = input.split("\n\n");

    let mut chunks_iter = chunks.into_iter();

    let nums: Vec<i32> = chunks_iter
        .next()
        .unwrap()
        .split(",")
        .map(|l| {
            l.parse().unwrap()
        })
        .collect();

    let mut boards: Vec<Board> = Vec::new();

    for (board_idx, chunk) in chunks_iter.enumerate() {
        let mut matrix = [[0; 5]; 5];

        let mut num_to_point = HashMap::new();

        for (y, row) in chunk.split("\n").into_iter().enumerate() {
            let col_nums: Vec<i32> = space_re
                .split(row.trim())
                .map(|c| { c.parse().unwrap() })
                .collect();

            for (x, num) in col_nums.into_iter().enumerate() {
                let point = Point { x: x as i32, y: y as i32 };

                matrix[x][y] = num;
                num_to_point.insert(num, point);
            }
        }

        let board = Board {
            name: board_idx as i32,
            matrix,
            num_to_point,
            seen_points: HashSet::new(),
        };

        boards.push(board);
    }

    println!("Boards length {}", boards.len());
    let num_boards = boards.len();
    let mut winning_boards = HashSet::new();

    for num in nums {
        for board_idx in 0..boards.len() {
            boards[board_idx].add_num(num);

            if boards[board_idx].is_winner() {
                winning_boards.insert(board_idx);

                if winning_boards.len() == num_boards {
                    println!("Last board is {}", boards[board_idx].name);
                    return boards[board_idx].winning_score(num);
                }
            }
        }
    }

    10
}
