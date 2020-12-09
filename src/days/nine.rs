pub fn nine() {
    let input = std::fs::read_to_string("input/nine.txt").unwrap();

    let numbers = input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    numbers
        .iter()
        .enumerate()
        .skip(25)
        .filter(|&(i, &num)| {
            let count = numbers
                .iter()
                .skip(i - 25)
                .take(25)
                .filter(|&&num1| {
                    numbers
                        .iter()
                        .skip(i - 25)
                        .take(25)
                        .filter(|&&num2| num1 + num2 == num)
                        .count()
                        != 0
                })
                .count();

            if count == 0 {
                for j in 0..=i {
                    for k in 0..=i {
                        if numbers.iter().skip(j).take(k).sum::<usize>() == num {
                            let min = numbers.iter().skip(j).take(k).min().unwrap();
                            let max = numbers.iter().skip(j).take(k).max().unwrap();
                            println!("min: {}, max: {}, sum: {}", min, max, min + max);
                        }
                    }
                }
                
            };

            count == 0
        })
        .for_each(|(_, num)| {
            println!("num: {}", num);
        });
}
