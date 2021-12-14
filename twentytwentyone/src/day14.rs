use counter::Counter;
use std::collections::HashMap;

#[aoc(day14, part1, for_hash)]
pub fn solve_part1(input: &str) -> i32 {
    let mut lines = input.lines();

    let mut template: Vec<char> = lines.next().unwrap().to_owned().chars().collect();
    println!("template {:?}", template);

    lines.next();

    let mut pattern_to_val: HashMap<(char, char), char> = HashMap::new();

    for line in lines {
        let mut parts = line.split(" -> ");

        let from_pattern = parts.next().unwrap();
        let mut from_chars = from_pattern.chars();

        let from1 = from_chars.next().unwrap();
        let from2 = from_chars.next().unwrap();

        let to = parts.next().unwrap();

        pattern_to_val.insert((from1, from2), to.chars().next().unwrap());
    }

    // println!("rules {:?}", pattern_to_val);

    let steps = 10;
    for step in 1..(steps + 1) {
        // println!("step {}", step);

        let template_length = &template.len();

        // Iterate from end of array to beginning
        for offset in 2..(template_length + 1) {
            let idx = template_length - offset;
            if let Some(&to_insert) = pattern_to_val.get(&(template[idx], template[idx + 1])) {
                // println!("at idx {} {}", idx, to_insert);
                template.insert(idx + 1, to_insert);
            }
        }

        // let temp_str: String = template.iter().collect();
        // println!("hi {}", temp_str);
    }

    let char_counts = template.iter().collect::<Counter<_>>();
    let most_common = char_counts.most_common_ordered();
    // println!("hi {:?}", most_common);

    let &fst = most_common.first().unwrap();
    let &lst = most_common.last().unwrap();

    (fst.1 - lst.1) as i32
}


#[aoc(day14, part2, for_hash)]
pub fn solve_part2(input: &str) -> i32 {
    let mut lines = input.lines();

    let mut template: Vec<char> = lines.next().unwrap().to_owned().chars().collect();
    lines.next();

    let mut pattern_to_count: HashMap<(char, char), i64> = HashMap::new();

    for idx in 0..(template.len() - 1) {
        let pattern = (template[idx], template[idx+1]);
        let entry = pattern_to_count.entry(pattern).or_insert(0);
        *entry += 1;
    }

    let mut pattern_to_val: HashMap<(char, char), char> = HashMap::new();

    for line in lines {
        let mut parts = line.split(" -> ");

        let from_pattern = parts.next().unwrap();
        let mut from_chars = from_pattern.chars();

        let from1 = from_chars.next().unwrap();
        let from2 = from_chars.next().unwrap();

        let to = parts.next().unwrap();

        pattern_to_val.insert((from1, from2), to.chars().next().unwrap());
    }

    println!("rules {:?}", pattern_to_val);
    println!("count {:?}", pattern_to_count);

    let steps = 40;
    for step in 1..(steps + 1) {
        println!("step {}", step);
        println!("count {:?}", pattern_to_count);

        let mut new_pattern_to_count = HashMap::new();

        // Iterate from end of array to beginning
        for (&pattern, &val) in pattern_to_val.iter() {
            let count = pattern_to_count.get(&pattern).unwrap_or(&0);
            if count == &0 {
                continue;
            }

            let p1 = (pattern.0, val);
            let entry = new_pattern_to_count.entry(p1).or_insert(0);
            *entry += count;

            let p2 = (val, pattern.1);
            let entry = new_pattern_to_count.entry(p2).or_insert(0);
            *entry += count;
        }

        pattern_to_count = new_pattern_to_count;

        // let temp_str: String = template.iter().collect();
        // println!("hi {}", temp_str);
    }

    let mut char_to_count = HashMap::new();
    for (&pattern, &count) in pattern_to_count.iter() {
        let entry = char_to_count.entry(pattern.0).or_insert(0);
        *entry += count;

        let entry = char_to_count.entry(pattern.1).or_insert(0);
        *entry += count;
    }

    let entry = char_to_count.entry('N').or_insert(0);
    println !("{}", entry);
    let entry = char_to_count.entry('B').or_insert(0);
    println !("{}", entry);
    let entry = char_to_count.entry('P').or_insert(0);
    println !("{}", entry);
    let entry = char_to_count.entry('C').or_insert(0);
    *entry += 1;
    println !("{}", entry);
    let entry = char_to_count.entry('H').or_insert(0);
    *entry += 1;
    println !("{}", entry);

    println!("counts {:?}", char_to_count);

    10
}

