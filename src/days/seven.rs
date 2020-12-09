fn find_bag<'a>(input: &'a str, name: &str, bags: &mut Vec<&'a str>) {
    input
        .lines()
        .filter(|&line| !line.starts_with(name) && line.contains(name))
        .for_each(|line| {
            let (bag, _) = line.split_once(" contain ").unwrap();
            let bag = bag.trim_end_matches('.');
            let bag = bag.trim_end_matches('s');
            if !bags.contains(&bag) {
                bags.push(bag)
            }
            find_bag(input, bag, bags)
        });
}

fn count_bag<'a>(input: &'a str, name: &str) -> usize {
    let name_line = input.lines().find(|&line| line.starts_with(name)).unwrap();
    let (_, rest) = name_line.split_once(" contain ").unwrap();
    let rest = rest.trim_end_matches('.');
    if rest == "no other bags" {
        return 1;
    };
    println!("rest: {}", rest);
    rest.split(", ")
        .map(|bag| {
            let (count, bag) = bag.split_once(' ').unwrap();
            count.parse::<usize>().unwrap() * count_bag(input, bag)
        })
        .sum::<usize>()
        + 1
}

pub fn seven() {
    let input = std::fs::read_to_string("input/seven.txt").unwrap();

    let mut colors = Vec::<&str>::new();

    find_bag(input.as_str(), "shiny gold bag", &mut colors);
    println!("bags: {}", colors.len());

    let bag_count = count_bag(input.as_str(), "shiny gold bag") - 1;
    println!("bag_cout: {}", bag_count);
}
