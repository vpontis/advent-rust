use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone, Hash)]
enum CaveType {
    Start,
    End,
    Small,
    Large,
}

#[derive(Debug, Clone, Hash)]
struct Cave {
    name: String,
    ctype: CaveType,
    neighbors: Vec<String>,
}

impl Cave {
    fn fromName(val: &str) -> Self {
        let ctype: CaveType;
        if val == "start" {
            ctype = CaveType::Start;
        } else if val == "end" {
            ctype = CaveType::End
        } else if val.to_ascii_lowercase() == val {
            ctype = CaveType::Small
        } else {
            ctype = CaveType::Large
        }

        let mut cave = Cave {
            name: val.to_owned(),
            ctype,
            neighbors: Vec::new(),
        };

        cave
    }

    fn add_neighbor(&mut self, neighbor: &str) {
        self.neighbors.push(neighbor.to_owned());
    }
}

impl fmt::Display for Cave {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cave({})", self.name)
    }
}

impl PartialEq<Self> for Cave {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Cave {}

#[derive(Hash)]
struct Path {
    caves: Vec<String>,
    has_small_twice: bool,
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Path({}, small={})", self.caves.join(","), self.has_small_twice)
    }
}

type CaveMap = HashMap<String, Cave>;

impl Path {
    fn next_paths(&self, cave_map: &CaveMap) -> Vec<Path> {
        let last_cave_name = self.caves.last().unwrap();
        let last_cave = cave_map.get(last_cave_name).unwrap();
        // println!("checking {}", self);

        let new_names: Vec<&String> = last_cave
            .neighbors
            .iter()
            .filter(|&str| {
                if str == "start" {
                    return false;
                }

                if str.to_ascii_lowercase() == str.to_owned() {
                    let count = self
                        .caves
                        .iter()
                        .filter(|&c|  c == str)
                        .count();

                    // println!("count of {} is {} and small {}", str, count, self.has_small_twice);

                    if self.has_small_twice == false {
                        return count <= 1;
                    } else {
                        return count == 0;
                    }
                }

                true
            })
            .collect();


        let new_paths: Vec<Path> = new_names
            .iter()
            .map(|&c| {
                let mut new_caves = self.caves.clone();
                new_caves.push(c.to_owned());
                let has_small_twice = self.has_small_twice || (self.caves.contains(c) && c.to_ascii_lowercase() == c.to_owned());

                Path { caves: new_caves, has_small_twice }
            })
            .collect();

        new_paths
    }
}

#[aoc(day12, part1, for_hash)]
pub fn solve_part1(input: &str) -> i32 {
    let mut cave_map: CaveMap = HashMap::new();

    for line in input.lines() {
        let mut line_parts = line.split("-").into_iter();
        let from = line_parts.next().unwrap();
        let to = line_parts.next().unwrap();

        let cave = cave_map.entry(from.to_owned()).or_insert(
            Cave::fromName(from)
        );
        cave.add_neighbor(to);

        let cave = cave_map.entry(to.to_owned()).or_insert(
            Cave::fromName(to)
        );
        cave.add_neighbor(from);
    }

    let mut paths = vec![
        Path {
            caves: vec![String::from("start")],
            has_small_twice: false,
        }
    ];


    println!("name to cave {:?}", cave_map);

    let mut winners = 0;

    while paths.is_empty() == false {
        let path = paths.pop().unwrap();

        if path.caves.last().unwrap() == "end" {
            println!("path {}", path);
            winners += 1;
            continue;
        }

        let next_paths = path.next_paths(&cave_map);


        for p in next_paths {
            paths.push(p);
        }
    }


    // get list of paths
    // each path can generate other paths
    // seen paths


    winners
}

