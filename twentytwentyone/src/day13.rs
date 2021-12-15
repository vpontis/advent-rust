use std::collections::{HashMap, HashSet};

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

#[aoc(day13, part1, for_hash)]
pub fn solve_part1(input: &str) -> i32 {
    let mut parts = input.split("\n\n");

    let points_str = parts.next().unwrap();
    let mut points: Vec<Point> = Vec::new();

    for line in points_str.lines() {
        let mut parts = line.split(",");
        let x: i32 = parts.next().unwrap().parse().unwrap();
        let y: i32 = parts.next().unwrap().parse().unwrap();

        points.push(Point { x, y });
    }

    println!("Points {:?}", points);

    let folds_str = parts.next().unwrap();
    for fold in folds_str.lines() {
        println!("fold {}", fold);
        let mut parts = fold.split("=");

        let instruction = parts.next().unwrap();
        let val: i32 = parts.next().unwrap().parse().unwrap();

        if instruction.contains("x") {
            for point in points.iter_mut() {
                if point.x >= val {
                    let delta = point.x - val;
                    println!("folding {} {}", point.x, val - delta);
                    point.x = val - delta;
                }
            }
        } else {
            for point in points.iter_mut() {
                if point.y >= val {
                    let delta = point.y - val;
                    println!("folding {} {}", point.y, val - delta);
                    point.y = val - delta;
                }
            }
        }

        let mut point_set = HashSet::new();
        for point in points.iter() {
            point_set.insert(point);
        }
    }

    let mut point_set = HashSet::new();
    for point in points.iter() {
        point_set.insert(point);
    }
    println!("Points {:?}", points);

    for y in 0..100 {
        let mut row: String = "".to_owned();
        for x in 0..100 {
            if point_set.contains(&Point {x: x as i32, y: y as i32}) {
                row += "#";
            } else {
                row += ".";
            }
        }
        println!("{}", row);
    }


    point_set.len() as i32
}

