pub fn six() {
    let input = std::fs::read_to_string("input/six.txt").unwrap();

    let unique_questions: usize = input
        .split("\n\n")
        .map(|line| line.replace("\n", ""))
        .map(|line| {
            ('a'..='z')
                .filter(|&question| line.contains(question))
                .count()
        })
        .sum();
    println!("unique_questions: {}", unique_questions);

    let group_questions: usize = input
        .split("\n\n")
        .map(|group| {
            ('a'..='z')
                .filter(|&question| {
                    group
                        .lines()
                        .filter(|group_member| !group_member.contains(question))
                        .count()
                        == 0
                })
                .count()
        })
        .sum();
    println!("group_questions : {}", group_questions);
}
