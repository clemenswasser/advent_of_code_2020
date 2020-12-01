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

fn main() {
    day_one();
}
