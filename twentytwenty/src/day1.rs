#[aoc(day1, part1, for_hash)]
pub fn solve_part1(input: &str) -> i32 {
    let nums: Vec<i32> = input
        .lines()
        .map(|line| {
            println!("line {}", line);
            return line.trim().parse::<i32>().unwrap();
        })
        .collect();

    for a in nums.iter() {
        for b in nums.iter() {
           if a == b {
               continue
           }

            if a + b == 2020 {
                println!("{}", a*b);
                return a * b;
            }
        }
    }

    unreachable!()
}

#[aoc(day1, part2, for_hash)]
pub fn solve_part2(input: &str) -> i32 {
    let nums: Vec<i32> = input
        .lines()
        .map(|line| {
            println!("line {}", line);
            return line.trim().parse::<i32>().unwrap();
        })
        .collect();

    for a in nums.iter() {
        for b in nums.iter() {
            for c in nums.iter() {
                if a + b + c == 2020 {
                    println!("{}", a * b * c);
                    return a * b * c;
                }
            }
        }
    }

    unreachable!()
}
