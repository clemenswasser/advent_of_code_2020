pub fn five() {
    let input = std::fs::read_to_string("input/five.txt").unwrap();
    let ids = input
        .lines()
        .map(|line| {
            line.chars()
                .enumerate()
                .fold((0, 0), |(row, column), (i, c)| match c {
                    'B' => (row | 64 >> i, column),
                    'F' => (row, column),
                    'R' => (row, column | 4 >> (i - 7)),
                    'L' => (row, column),
                    _ => panic!("Invalid Character!"),
                })
        })
        .map(|(row, column)| row * 8 + column)
        .collect::<Vec<_>>();

    let max_id = *ids.iter().max().unwrap();
    let min_id = *ids.iter().min().unwrap();

    println!("max_id: {}", max_id);

    (min_id + 1..max_id).for_each(|id| {
        if !ids.contains(&id) {
            if ids.contains(&(id + 1)) {
                if ids.contains(&(id - 1)) {
                    println!("my_id : {}", id)
                }
            }
        }
    });
}
