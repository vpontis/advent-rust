use std::collections::{HashMap, HashSet};

type Grid = Vec<Vec<usize>>;

#[aoc(day11, part1, for_hash)]
pub fn solve_part1(input: &str) -> i32 {
    let mut grid: Vec<Vec<usize>> = Vec::new();

    for line in input.lines() {
        let mut row = Vec::new();
        for char in line.chars() {
            row.push(char.to_digit(10).unwrap() as usize);
        }
        grid.push(row);
    }

    let steps = 10000;
    let mut num_flashes: i32 = 0;

    let row_len = grid.len();
    let col_len = grid[0].len();

    for step in 1..(steps + 1) {
        println!("step {}", step);

        let mut flashed = HashSet::new();
        let mut will_flash = Vec::new();

        for y in 0..row_len {
            for x in 0..col_len {
                grid[y][x] += 1;
                if grid[y][x] > 9 {
                    will_flash.push(Point { x, y })
                }
            }
        }

        while will_flash.is_empty() == false {
            let flash_point = will_flash.pop().unwrap();
            flashed.insert(flash_point);

            // Get all of the neighbors
            for neighbor in get_neighbors(&flash_point, &grid) {
                grid[neighbor.y][neighbor.x] += 1;
                if grid[neighbor.y][neighbor.x] > 9
                    && !flashed.contains(&neighbor)
                    && !will_flash.contains(&neighbor) {
                    will_flash.push(neighbor);
                }
            }
        }

        if flashed.len() == row_len * col_len {
            return step;
        }

        num_flashes += flashed.len() as i32;
        for p in flashed.iter() {
            grid[p.y][p.x] = 0;
        }
        // println!("flashes {}", num_flashes);
        // for row in &grid {
        //     println!("{:?}", row);
        // }
    }

    num_flashes
}

fn get_neighbors(point: &Point, grid: &Grid) -> Vec<Point> {
    let mut neighbors: Vec<Point> = Vec::new();

    for mut dx in 0..3 {
        for mut dy in 0..3 {
            if dy == dx && dy == 1{
                continue;
            }

            if point.y + dy == 0 || point.x + dx == 0 {
               continue;
            }

            let newy = point.y + dy - 1;
            let newx = point.x + dx - 1;

            if newy < 0 || newx < 0 {
                continue;
            }

            if let Some(row) = grid.get(newy as usize) {
                if let Some(&v) = row.get(newx as usize) {
                    neighbors.push(Point { x: newx, y: newy })
                }
            }
        }
    }

    neighbors
}


#[derive(Hash, Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl PartialEq<Self> for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

