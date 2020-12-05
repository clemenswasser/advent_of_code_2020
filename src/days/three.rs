pub fn three() {
    let input = std::fs::read_to_string("input/three.txt").unwrap();

    let trees_1r_1d = input
        .lines()
        .skip(1)
        .enumerate()
        .filter(|(i, line)| {
            let c_index = (i + 1) % line.len();
            let c = line.chars().nth(c_index).unwrap();
            c == '#'
        })
        .count();

    let trees_3r_1d = input
        .lines()
        .skip(1)
        .enumerate()
        .filter(|(i, line)| {
            let c_index = ((i + 1) * 3) % line.len();
            let c = line.chars().nth(c_index).unwrap();
            c == '#'
        })
        .count();

    let trees_5r_1d = input
        .lines()
        .skip(1)
        .enumerate()
        .filter(|(i, line)| {
            let c_index = ((i + 1) * 5) % line.len();
            let c = line.chars().nth(c_index).unwrap();
            c == '#'
        })
        .count();

    let trees_7r_1d = input
        .lines()
        .skip(1)
        .enumerate()
        .filter(|(i, line)| {
            let c_index = ((i + 1) * 7) % line.len();
            let c = line.chars().nth(c_index).unwrap();
            c == '#'
        })
        .count();

    let trees_1r_2d = input
        .lines()
        .skip(1)
        .enumerate()
        .filter(|(i, _)| (i + 1) % 2 == 0)
        .map(|(_, line)| line)
        .enumerate()
        .filter(|(i, line)| {
            let c_index = (i + 1) % line.len();
            let c = line.chars().nth(c_index).unwrap();
            c == '#'
        })
        .count();

    println!("trees_1r_1d: {}", trees_1r_1d);
    println!("trees_3r_1d: {}", trees_3r_1d);
    println!("trees_5r_1d: {}", trees_5r_1d);
    println!("trees_7r_1d: {}", trees_7r_1d);
    println!("trees_1r_2d: {}", trees_1r_2d);

    println!(
        "result     : {}",
        trees_1r_1d * trees_3r_1d * trees_5r_1d * trees_7r_1d * trees_1r_2d
    );
}
