pub fn two() {
    let file_string = std::fs::read_to_string("input/two.txt").unwrap();
    let policies_and_passwords = file_string
        .lines()
        .map(|line| {
            let (policy, password) = line.split_once(':').unwrap();
            let (range, expected_char) = policy.split_once(' ').unwrap();
            let (a, b) = range.split_once('-').unwrap();

            (
                (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()),
                expected_char.chars().next().unwrap(),
                password.trim(),
            )
        })
        .collect::<Vec<_>>();

    let valid_passwords_old_policy = policies_and_passwords
        .iter()
        .filter(|((min, max), expected_char, password)| {
            let char_count = password.chars().filter(|c| c == expected_char).count();

            char_count >= *min && char_count <= *max
        })
        .count();

    println!("valid_passwords_old_policy: {}", valid_passwords_old_policy);

    let valid_passwords_new_policy = policies_and_passwords
        .iter()
        .filter(|((pos1, pos2), expected_char, password)| {
            let pos1_char = password.chars().nth(pos1 - 1).unwrap();
            let pos2_char = password.chars().nth(pos2 - 1).unwrap();

            (pos1_char == *expected_char) ^ (pos2_char == *expected_char)
        })
        .count();

    println!("valid_passwords_new_policy: {}", valid_passwords_new_policy);
}
