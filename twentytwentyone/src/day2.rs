use regex::Regex;
#[aoc(day2, part1, for_hash)]
pub fn solve_part1(input: &str) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;

    let fwd_re = Regex::new(r"forward (\d)").unwrap();
    let up_re = Regex::new(r"up (\d)").unwrap();
    let down_re = Regex::new(r"down (\d)").unwrap();

    for line in input.lines() {
        let x = line[0] == 'f';
        let num = line[10..];

        if fwd_re.is_match(line) {
            let cap = fwd_re.captures(line).unwrap();
            let val = cap[1].parse::<i32>().unwrap();
            horizontal += val;
        }
        if up_re.is_match(line) {
            let cap = up_re.captures(line).unwrap();
            let val = cap[1].parse::<i32>().unwrap();
            depth -= val;
        }
        if down_re.is_match(line) {
            let cap = down_re.captures(line).unwrap();
            let val = cap[1].parse::<i32>().unwrap();
            depth += val;
        }

    }

    horizontal * depth
}

#[aoc(day2, part2, for_hash)]
pub fn solve_part2(input: &str) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    let fwd_re = Regex::new(r"forward (\d)").unwrap();
    let up_re = Regex::new(r"up (\d)").unwrap();
    let down_re = Regex::new(r"down (\d)").unwrap();

    for line in input.lines() {
        if fwd_re.is_match(line) {
            let cap = fwd_re.captures(line).unwrap();
            let val = cap[1].parse::<i32>().unwrap();
            depth += val * aim;
            horizontal += val;
        }
        if up_re.is_match(line) {
            let cap = up_re.captures(line).unwrap();
            let val = cap[1].parse::<i32>().unwrap();
            aim -= val;
        }
        if down_re.is_match(line) {
            let cap = down_re.captures(line).unwrap();
            let val = cap[1].parse::<i32>().unwrap();
            aim += val;
        }

    }

    horizontal * depth
}
