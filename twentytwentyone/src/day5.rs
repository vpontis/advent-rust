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


#[aoc(day5, part1, for_hash)]
pub fn solve_part1(input: &str) -> i32 {
    let line_re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();


    let mut point_to_value = HashMap::new();

    for line in input.lines() {
        let cap = line_re.captures(line).unwrap();

        let x1 = cap[1].parse::<i32>().unwrap();
        let y1 = cap[2].parse::<i32>().unwrap();
        let x2 = cap[3].parse::<i32>().unwrap();
        let y2 = cap[4].parse::<i32>().unwrap();

        if x1 == x2 {
            let miny = std::cmp::min(y1, y2);
            let maxy = std::cmp::max(y1, y2) + 1;
            for y in miny..maxy {
                let point = Point { x: x1, y };
                let mut seen = point_to_value.entry(point).or_insert(0);
                *seen += 1;
            }
        } else if y1 == y2 {
            let minx = std::cmp::min(x1, x2);
            let maxx = std::cmp::max(x1, x2) + 1;
            for x in minx..maxx {
                let point = Point { x, y: y1 };
                let mut seen = point_to_value.entry(point).or_insert(0);
                *seen += 1;
            }
        }
    }

    let mut result = 0;
    for (point, seen) in point_to_value {
        if seen > 1 {
            result += 1;
        }
    }

    result
}

#[aoc(day5, part2, for_hash)]
pub fn solve_part2(input: &str) -> i32 {
    let line_re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();


    let mut point_to_value = HashMap::new();

    for line in input.lines() {
        let cap = line_re.captures(line).unwrap();

        let x1 = cap[1].parse::<i32>().unwrap();
        let y1 = cap[2].parse::<i32>().unwrap();
        let x2 = cap[3].parse::<i32>().unwrap();
        let y2 = cap[4].parse::<i32>().unwrap();

        if x1 == x2 {
            let miny = std::cmp::min(y1, y2);
            let maxy = std::cmp::max(y1, y2) + 1;
            for y in miny..maxy {
                let point = Point { x: x1, y };
                let mut seen = point_to_value.entry(point).or_insert(0);
                *seen += 1;
            }
        } else if y1 == y2 {
            let minx = std::cmp::min(x1, x2);
            let maxx = std::cmp::max(x1, x2) + 1;
            for x in minx..maxx {
                let point = Point { x, y: y1 };
                let mut seen = point_to_value.entry(point).or_insert(0);
                *seen += 1;
            }
        } else {
            println!("here {},{} -> {},{}", x1, y1, x2, y2);

            // Get the slope of the line
            // slope = my/mx
            let ydiff = y2 - y1;
            let xdiff = x2 - x1;

            let mut slope = ydiff as f64 / xdiff as f64;

            println!("slope is {}", slope);

            let minx = std::cmp::min(x1, x2);
            let maxx = std::cmp::max(x1, x2) + 1;

            let inity = if minx == x1 { y1 } else { y2 };

            for x in minx..maxx {
                let xdelta = x - minx;
                let yval = xdelta as f64 * slope + inity as f64;

                if yval.fract() != 0.0 {
                    println!("continue?");
                    continue;
                }

                let point = Point { x, y: yval as i32 };
                println!("point {:?}", point);
                let mut seen = point_to_value.entry(point).or_insert(0);
                *seen += 1;
            }
        }
    }

    let mut result = 0;
    for (point, seen) in point_to_value {
        if seen > 1 {
            result += 1;
        }
    }

    result
}
