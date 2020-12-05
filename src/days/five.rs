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

    println!("max_id: {}", ids.iter().max().unwrap());

    (min_id + 1..max_id).for_each(|id| {
        if ids.iter().find(|&&i| i == id).is_none() {
            if ids.iter().find(|&&i| i == id + 1).is_some() {
                if ids.iter().find(|&&i| i == id - 1).is_some() {
                    println!("my_id : {}", id)
                }
            }
        }
    });
}
