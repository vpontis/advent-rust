#[aoc(day1, part1, for_hash)]
pub fn solve_part1(input: &str) -> i32 {
    let nums: Vec<i32> = input
        .lines()
        .map(|line| {
            return line.trim().parse::<i32>().unwrap();
        })
        .collect();

    let mut count = 0;
    for i in 1..nums.len() {
        if nums[i-1] < nums[i] {
            count += 1;
        }
    }

    count
}

#[aoc(day1, part2, for_hash)]
pub fn solve_part2(input: &str) -> i32 {
    let nums: Vec<i32> = input
        .lines()
        .map(|line| {
            return line.trim().parse::<i32>().unwrap();
        })
        .collect();

    let mut count = 0;
    for i in 3..nums.len() {
        let prevsum = nums[i-3] + nums[i-2] + nums[i-1];
        let cursum = nums[i-2] + nums[i-1] + nums[i];

        if prevsum < cursum {
            count += 1;
        }
    }

    count
}
