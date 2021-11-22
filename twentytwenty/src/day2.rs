use regex::Regex;

// 1-3 a: abcde
// 1-3 b: cdefg
// 2-9 c: ccccccccc
pub struct Password {
    min: u32,
    max: u32,
    letter: char,
    password: String,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Password> {
    let re = Regex::new(r"(\d+)\-(\d+) (\w): (\w+)").unwrap();

    input
        .lines()
        .map(|l| {
            let cap = re.captures(l).unwrap();
            let min = &cap[1].parse::<u32>().unwrap();
            let max = &cap[2].parse::<u32>().unwrap();
            let letter = cap[3].chars().next().unwrap();
            let password = String::from(&cap[4]);

            let pass = Password {
                min: min.clone(),
                max: max.clone(),
                letter,
                password,
            };

            return pass;
        })
        .collect()
}


#[aoc(day2, part1, for_hash)]
pub fn solve_part1(input: &Vec<Password>) -> i32 {
    let count = input.iter().filter(|&pass| {
        let mut count = 0;
        for char in pass.password.chars() {
            if char == pass.letter {
                count += 1;
            }
        }

        count >= pass.min && count <= pass.max
    }).count() ;

    println!("The answer is {}", count);
    count as i32
}


#[aoc(day2, part2, for_hash)]
pub fn solve_part2(input: &Vec<Password>) -> i32 {
    let count = input.iter().filter(|&pass| {
        let char1 = pass.password.as_bytes()[(pass.min as usize) - 1] as char;
        let char2 = pass.password.as_bytes()[(pass.max as usize) - 1] as char;

        (char1 == pass.letter && char2 != pass.letter) || (char1 != pass.letter && char2 == pass.letter)
    }).count() ;

    count as i32
}

