use std::collections::{HashMap, HashSet};

#[aoc(day9, part1, for_hash)]
pub fn solve_part1(input: &str) -> i32 {
    let mut grid: Vec<Vec<i32>> = Vec::new();

    for line in input.lines() {
        let mut row: Vec<i32> = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap() as i32)
        }
        grid.push(row);
    }

    let row_len = grid.len();
    let col_len = grid[0].len();


    let mut answer = 0;

    for y in 0..row_len {
        for x in 0..col_len {
            let mut is_low = true;

            let val = grid[y][x];

            for mut dx in 0..3 {
                for mut dy in 0..3 {
                    if dy == dx && dx == 1 {
                        continue;
                    }

                    let newy: i32 = y as i32 + dy as i32 - 1;
                    let newx: i32 = x as i32 + dx as i32 - 1;

                    if newy < 0 || newx < 0 {
                        continue;
                    }

                    if let Some(row) = grid.get(newy as usize) {
                        if let Some(&v) = row.get(newx as usize) {
                            if v <= val {
                                is_low = false;
                            }
                        }
                    }
                }
            }

            if is_low {
                answer += val + 1;
            }
        }
    }

    answer
}

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

type Grid = Vec<Vec<i32>>;

fn get_lowpoints(grid: &Vec<Vec<i32>>) -> Vec<Point> {
    let mut lowpoints: Vec<Point> = Vec::new();

    let row_len = grid.len();
    let col_len = grid[0].len();

    for y in 0..row_len {
        for x in 0..col_len {
            let mut is_low = true;

            let val = grid[y][x];

            for mut dx in 0..3 {
                for mut dy in 0..3 {
                    if dy == dx && dx == 1 {
                        continue;
                    }

                    let newy: i32 = y as i32 + dy as i32 - 1;
                    let newx: i32 = x as i32 + dx as i32 - 1;

                    if newy < 0 || newx < 0 {
                        continue;
                    }

                    if let Some(row) = grid.get(newy as usize) {
                        if let Some(&v) = row.get(newx as usize) {
                            if v <= val {
                                is_low = false;
                            }
                        }
                    }
                }
            }

            if is_low {
                lowpoints.push(Point { x: x as i32, y: y as i32 });
            }
        }
    }

    lowpoints
}

struct Neighbor {
    point: Point,
    val: i32,
}

fn get_neighbors(point: &Point, grid: &Grid) -> Vec<Neighbor> {
    let mut neighbors: Vec<Neighbor> = Vec::new();

    for mut dx in 0..3 {
        for mut dy in 0..3 {
            if dy == dx {
                continue;
            }
            if dy != 1 && dx != 1 {
                continue;
            }

            let newy: i32 = point.y as i32 + dy as i32 - 1;
            let newx: i32 = point.x as i32 + dx as i32 - 1;
            println!("old xy {},{} new xy {},{} hmm {},{}", point.x, point.y, newx, newy, dx, dy);

            if newy < 0 || newx < 0 {
                continue;
            }

            if let Some(row) = grid.get(newy as usize) {
                if let Some(&v) = row.get(newx as usize) {
                    neighbors.push(Neighbor {
                        point: Point {
                            x: newx,
                            y: newy,
                        },
                        val: v,
                    })
                }
            }
        }
    }

    neighbors
}

fn get_basin_size(point: &Point, grid: &Grid) -> i32 {
    let mut size: i32 = 0;
    let mut seen_points = HashSet::new();
    let mut queue: Vec<Point> = Vec::new();

    seen_points.insert(point.clone());
    queue.push(point.clone());
    size += 1;

    while &queue.is_empty() == &false {
        let new_point = queue.pop().unwrap();
        println!("point {:?}", new_point);

        let neighbors = get_neighbors(&new_point, &grid);

        for neighbor in neighbors {
            let npoint = neighbor.point;

            if !queue.contains(&npoint) && !seen_points.contains(&npoint) {
                if neighbor.val != 9 {
                    queue.push(npoint.clone());
                    size += 1;
                }
            }

            seen_points.insert(npoint.clone());
        }
    }

    println!("size is {}", size);

    size
}

#[aoc(day9, part2, for_hash)]
pub fn solve_part2(input: &str) -> i32 {
    let mut grid: Vec<Vec<i32>> = Vec::new();

    for line in input.lines() {
        let mut row: Vec<i32> = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap() as i32)
        }
        grid.push(row);
    }

    let lowpoints = get_lowpoints(&grid);
    println!("hmmm {:?}", lowpoints);

    let mut basin_sizes: Vec<i32> = Vec::new();

    for point in lowpoints {
        let basin_size = get_basin_size(&point, &grid);
        basin_sizes.push(basin_size);
    }
    // get basin size
    // do a search from that point until we hit 9s

    basin_sizes.sort();
    basin_sizes.reverse();
    basin_sizes.get(0..3).unwrap().iter().product()
}

