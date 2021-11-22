use regex::Regex;

#[aoc(day3, part1, for_hash)]
pub fn solve_part1(input: &str) -> i32 {
    let rows: Vec<Vec<bool>> = input
        .lines()
        .map(|line| {
            line.chars().map(|c| {
                c == '#'
            }).collect()
        })
        .collect();

    // let num_rows = rows.len();
    // let row_length = rows[0].len();
    //
    // let mut x = 0;
    // let mut trees = 0;
    // for i in (0..num_rows) {
    //     println!("hi i {} {}", i, num_rows);
    //     if rows[i][x] {
    //         trees += 1;
    //     }
    //
    //     x = (x + 3) % row_length;
    // }
    //
    trees_on_slope(&rows, Slope { right: 3, down: 1 })
}

struct Slope {
    right: usize,
    down: usize
}

fn trees_on_slope(rows: &Vec<Vec<bool>>, slope: Slope) -> i32 {
    let num_rows = rows.len();
    let row_length = rows[0].len();

    let mut x = 0;
    let mut trees = 0;
    let mut y = 0;

    while y < num_rows {
        if rows[y][x] {
            trees += 1;
        }

        x = (x + slope.right) % row_length;
        y = y + slope.down;
    }

    return trees;
}

#[aoc(day3, part2, for_hash)]
pub fn solve_part2(input: &str) -> i32 {
    let rows: Vec<Vec<bool>> = input
        .lines()
        .map(|line| {
            line.chars().map(|c| {
                c == '#'
            }).collect()
        })
        .collect();

    trees_on_slope(&rows, Slope { right: 1, down: 1 }) *
        trees_on_slope(&rows, Slope { right: 3, down: 1 }) *
        trees_on_slope(&rows, Slope { right: 5, down: 1 }) *
        trees_on_slope(&rows, Slope { right: 7, down: 1 }) *
        trees_on_slope(&rows, Slope { right: 1, down: 2 })
}
