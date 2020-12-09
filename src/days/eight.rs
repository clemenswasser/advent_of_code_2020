pub fn eight() {
    let input = std::fs::read_to_string("input/eight.txt").unwrap();
    let mut executed_insns = Vec::<usize>::new();
    let mut insn_index = 0;
    let mut acc = 0;

    while !executed_insns.contains(&insn_index) {
        executed_insns.push(insn_index);
        let (insn, num_s) = input
            .lines()
            .nth(insn_index)
            .unwrap()
            .split_once(' ')
            .unwrap();

        let num = num_s.parse::<isize>().unwrap();
        match insn {
            "jmp" => {
                insn_index = (insn_index as isize + num) as usize;
                continue;
            }
            "acc" => {
                acc += num;
            }
            _ => {}
        }
        insn_index += 1;
    }

    println!("acc: {}", acc);

    for (line, _) in input
        .lines()
        .enumerate()
        .filter(|(_, line)| line.starts_with("nop"))
    {
        let mut executed_insns = Vec::<usize>::new();
        let mut insn_index = 0;
        let mut acc = 0;
        while !executed_insns.contains(&insn_index) {
            executed_insns.push(insn_index);

            if insn_index == input.lines().count() - 1 {
                println!("clean_acc: {}", acc);
                return;
            }

            let (insn, num_s) = input
                .lines()
                .nth(insn_index)
                .unwrap()
                .split_once(' ')
                .unwrap();
            let num = num_s.parse::<isize>().unwrap();

            if insn_index == line {
                insn_index = (insn_index as isize + num) as usize;
                continue;
            }

            match insn {
                "jmp" => {
                    insn_index = (insn_index as isize + num) as usize;
                    continue;
                }
                "acc" => {
                    acc += num;
                }
                _ => {}
            }
            insn_index += 1;
        }
    }

    for (line, _) in input
        .lines()
        .enumerate()
        .filter(|(_, line)| line.starts_with("jmp"))
    {
        let mut executed_insns = Vec::<usize>::new();
        let mut insn_index = 0;
        let mut acc = 0;
        while !executed_insns.contains(&insn_index) {
            executed_insns.push(insn_index);

            if insn_index == input.lines().count() - 1 {
                println!("clean_acc: {}", acc);
                return;
            }

            let (insn, num_s) = input
                .lines()
                .nth(insn_index)
                .unwrap()
                .split_once(' ')
                .unwrap();
            let num = num_s.parse::<isize>().unwrap();

            if insn_index == line {
                insn_index += 1;
                continue;
            }

            match insn {
                "jmp" => {
                    insn_index = (insn_index as isize + num) as usize;
                    continue;
                }
                "acc" => {
                    acc += num;
                }
                _ => {}
            }
            insn_index += 1;
        }
    }
}
