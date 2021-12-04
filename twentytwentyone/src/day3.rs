use std::collections::HashSet;

#[aoc(day3, part1, for_hash)]
pub fn solve_part1(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();

    let line_length = lines[0].len();
    let num_lines = lines.len();

    let mut gamma = "".to_owned();
    let mut epsilon = "".to_owned();

    for char_idx in 0..line_length {
        let mut num_ones = 0;
        let mut num_zeros = 0;

        for line in input.lines() {
            let char = line.as_bytes()[char_idx] as char;
            match char {
                '1' => { num_ones += 1 }
                '0' => { num_zeros += 1 }
                _ => panic!("Found invalid character.")
            }
        }

        if num_zeros > num_ones {
            gamma = gamma.to_owned() + "0";
            epsilon = epsilon.to_owned() + "1";
        } else {
            gamma = gamma.to_owned() + "1";
            epsilon = epsilon.to_owned() + "0";
        }
    }

    println!("gamma {} epsilon {}", gamma, epsilon);

    (isize::from_str_radix(gamma.as_str(), 2).unwrap() as i32) *
        (isize::from_str_radix(epsilon.as_str(), 2).unwrap() as i32)
}

#[aoc(day3, part2, for_hash)]
pub fn solve_part2(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();

    let line_length = lines[0].len();

    let mut valid_co2 = Vec::new();
    let mut valid_oxygen = Vec::new();
    let mut oxygens = "".to_owned();
    let mut co2s = "".to_owned();

    for i in 0..lines.len() {
        valid_co2.push(i);
        valid_oxygen.push(i);
    }
    println!("hi {:?}", valid_co2);
    println!("hi {:?}", valid_oxygen);

    let mut char_idx = 0;

    while valid_oxygen.len() > 1 && char_idx < line_length {
        let mut num_ones = 0;
        let mut num_zeros = 0;

        for line_idx in valid_oxygen.iter().cloned() {
            let c = lines[line_idx].as_bytes()[char_idx] as char;

            match c {
                '1' => { num_ones += 1 }
                '0' => { num_zeros += 1 }
                _ => { panic!("hiiiiiiii") }
            }

        }

        println!("hi zeros {} ones {}", num_zeros, num_ones);

        if valid_oxygen.len() > 1 {
            let mut new_valid_oxygen = Vec::new();

            for idx in valid_oxygen.iter().cloned() {
                let c = lines[idx].as_bytes()[char_idx] as char;

                if num_ones >= num_zeros {
                    if c == '1' {
                        new_valid_oxygen.push(idx);
                    }
                } else {
                    if c == '0' {
                        new_valid_oxygen.push(idx);
                    }
                }
            }
            valid_oxygen = new_valid_oxygen;
        }

        if num_ones >= num_zeros {
            oxygens = oxygens.to_owned() + "1";
        } else {
            oxygens = oxygens.to_owned() + "0";
        }


        char_idx += 1;
        println!("char idx {}", char_idx);
    }

    println!("oxygen {:?}", valid_oxygen);
    let mut oxygen = isize::from_str_radix(lines[valid_oxygen.into_iter().next().unwrap()], 2).unwrap() as i32;
    println!("fuck {}", oxygen);


    char_idx = 0;
    println!("HOW BIG IS MY CO2 {}", valid_co2.len());
    while valid_co2.len() > 1 && char_idx < line_length {
        let mut num_ones = 0;
        let mut num_zeros = 0;

        for line_idx in valid_co2.iter().cloned() {
            let c = lines[line_idx].as_bytes()[char_idx] as char;

            match c {
                '1' => { num_ones += 1 }
                '0' => { num_zeros += 1 }
                _ => { panic!("hiiiiiiii") }
            }

        }

        println!("hi zeros {} ones {}", num_zeros, num_ones);

        if valid_co2.len() > 1 {
            let mut new_valid = Vec::new();

            for idx in valid_co2.iter().cloned() {
                let c = lines[idx].as_bytes()[char_idx] as char;
                println!("lopp {}", num_ones > num_zeros);

                if num_ones >= num_zeros {
                    if c == '0' {
                        println!("IN THIS WORLD");
                        new_valid.push(idx);
                    }
                } else {
                    if c == '1' {
                        new_valid.push(idx);
                    }
                }
            }
            println!("why so small!!! {:?}", new_valid);
            valid_co2 = new_valid;
        }

        if num_ones > num_zeros {
            co2s = co2s.to_owned() + "1";
        } else {
            co2s = co2s.to_owned() + "0";
        }

        char_idx += 1;
        println!("char idx {}", char_idx);
    }

    println!("co2 {:?}", valid_co2);
    println!("co2s {}", co2s);

    let mut co2 = isize::from_str_radix(lines[valid_co2.into_iter().next().unwrap()], 2).unwrap() as i32;
    println!("fuck co2 {}", co2);


    println!("oxygen {} co2 {}", oxygen, co2);

    co2 * oxygen
}
