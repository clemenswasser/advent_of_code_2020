#![feature(str_split_once)]

fn day_one() {
    let numbers = std::fs::read_to_string("input/one.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    'outer1: for a in &numbers {
        for b in &numbers {
            if a + b == 2020 {
                println!("a: {}, b: {}, a * b * c: {}", a, b, a * b);
                break 'outer1;
            }
        }
    }

    'outer2: for a in &numbers {
        for b in &numbers {
            for c in &numbers {
                if a + b + c == 2020 {
                    println!("a: {}, b: {}, c: {}, a * b * c: {}", a, b, c, a * b * c);
                    break 'outer2;
                }
            }
        }
    }
}

fn day_two() {
    let file_string = std::fs::read_to_string("input/two.txt").unwrap();
    let pairs = file_string
        .lines()
        .map(|line| {
            let (policy, password) = line.split_once(':').unwrap();
            
            (policy.trim(), password.trim())
        })
        .collect::<Vec<_>>();

    let valid_passwords_old_policy = pairs
        .iter()
        .filter(|(policy, password)| {
            let (range, char) = policy.split_once(' ').unwrap();

            let (min, max) = {
                let (min_s, max_s) = range.split_once('-').unwrap();
                (
                    min_s.parse::<usize>().unwrap(),
                    max_s.parse::<usize>().unwrap(),
                )
            };

            let char_count = password
                .chars()
                .filter(|c| *c == char.chars().next().unwrap())
                .count();

            char_count >= min && char_count <= max
        })
        .count();

    println!("valid_passwords_old_policy: {}", valid_passwords_old_policy);

    let valid_passwords_new_policy = pairs
        .iter()
        .filter(|(policy, password)| {
            let (range, char) = {
                let (range_s, char_s) = policy.split_once(' ').unwrap();
                (range_s, char_s.chars().next().unwrap())
            };

            let (pos1, pos2) = {
                let (min_s, max_s) = range.split_once('-').unwrap();
                (
                    min_s.parse::<usize>().unwrap(),
                    max_s.parse::<usize>().unwrap(),
                )
            };

            password.chars().nth(pos1 - 1).unwrap() == char
                && password.chars().nth(pos2 - 1).unwrap() != char
                || password.chars().nth(pos1 - 1).unwrap() != char
                    && password.chars().nth(pos2 - 1).unwrap() == char
        })
        .count();

    println!("valid_passwords_new_policy: {}", valid_passwords_new_policy);
}

fn main() {
    println!("Day One:");
    day_one();
    println!();

    println!("Day Two:");
    day_two();
    println!();
}
