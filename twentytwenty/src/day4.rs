use regex::Regex;

// #[aoc(day4, part1, for_hash)]
pub fn solve_part1(input: &str) -> i32 {
    let mut good_count = 0;

    let mut byr = false;
    let mut iyr = false;
    let mut eyr = false;
    let mut hgt = false;
    let mut hcl = false;
    let mut ecl = false;
    let mut pid = false;
    let mut cid = false;

    for line in input.lines() {
        let is_blank = line.trim() == "";

        if !is_blank {
            byr = byr || line.contains("byr:");
            iyr = iyr || line.contains("iyr:");
            eyr = eyr || line.contains("eyr:");
            hgt = hgt || line.contains("hgt:");
            hcl = hcl || line.contains("hcl:");
            ecl = ecl || line.contains("ecl:");
            pid = pid || line.contains("pid:");
            cid = cid || line.contains("cid:");
        }

        if is_blank {
            if byr && iyr && eyr && hgt && hcl && ecl && pid {
                good_count += 1;
            }

            byr = false;
            iyr = false;
            eyr = false;
            hgt = false;
            hcl = false;
            ecl = false;
            pid = false;
            cid = false;
        }
    }

    if byr && iyr && eyr && hgt && hcl && ecl && pid {
        good_count += 1;
    }

    good_count
}

#[aoc(day4, part2, for_hash)]
pub fn solve_part2(input: &str) -> i32 {
    let mut good_count = 0;

    let byr_re = Regex::new(r"byr:(\d{4})\b").unwrap();
    let iyr_re = Regex::new(r"iyr:(\d{4})\b").unwrap();
    let eyr_re = Regex::new(r"eyr:(\d{4})\b").unwrap();
    let hgt_re = Regex::new(r"hgt:(\d+)(cm|in)\b").unwrap();
    let hcl_re = Regex::new(r"hcl:(#[0-9a-f]{6})\b").unwrap();
    let ecl_re = Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)\b").unwrap();
    let pid_re = Regex::new(r"pid:\d{9}\b").unwrap();

    let mut byr = false;
    let mut iyr = false;
    let mut eyr = false;
    let mut hgt = false;
    let mut hcl = false;
    let mut ecl = false;
    let mut pid = false;

    for line in input.lines() {
        let is_blank = line.trim() == "";

        if !is_blank {
            if byr_re.is_match(line) {
                let cap = byr_re.captures(line).unwrap();
                let year = cap[1].parse::<i32>().unwrap();
                byr = byr || (year >= 1920 && year <= 2002)
            }

            if iyr_re.is_match(line) {
                let cap = iyr_re.captures(line).unwrap();
                let year = cap[1].parse::<i32>().unwrap();
                iyr = iyr || (year >= 2010 && year <= 2020)
            }

            if eyr_re.is_match(line) {
                let cap = eyr_re.captures(line).unwrap();
                let year = cap[1].parse::<i32>().unwrap();
                eyr = eyr || (year >= 2020 && year <= 2030)
            }

            if hgt_re.is_match(line) {
                let cap = hgt_re.captures(line).unwrap();
                let unit = &cap[2];
                let val = cap[1].parse::<i32>().unwrap();

                if unit == "cm" {
                    hgt = hgt || (val >= 150 && val <= 193);
                }
                if unit == "in" {
                    hgt = hgt || (val >= 59 && val <= 76);
                }
            }

            hcl = hcl || hcl_re.is_match(line);
            ecl = ecl || ecl_re.is_match(line);
            pid = pid || pid_re.is_match(line);
        }

        if is_blank {
            if byr && iyr && eyr && hgt && hcl && ecl && pid {
                good_count += 1;
            }

            byr = false;
            iyr = false;
            eyr = false;
            hgt = false;
            hcl = false;
            ecl = false;
            pid = false;
        }
    }

    if byr && iyr && eyr && hgt && hcl && ecl && pid {
        good_count += 1;
    }

    good_count
}

