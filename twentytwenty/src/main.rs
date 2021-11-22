// Read the user input
// Find two entries that sum to 2020, then multiply them
use std::fs;

fn main() {
    let filename = "1.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let lines = contents.split("\n");

    let mut nums: Vec<i32> = Vec::new();

    println!("{}", contents);

    for line in lines {
        println!("line {}", line);
        let num: i32 = match line.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        nums.push(num);
    }

    for a in nums.iter() {
        for b in nums.iter() {
            if a == b {
                continue
            }

            if a + b == 2020 {
                println!("a {}, b {}, a*b {}", a, b, a*b)
            }
        }
    }

    println!("Hello, world!");
}
