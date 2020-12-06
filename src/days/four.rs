use core::panic;

pub fn four() {
    let input = std::fs::read_to_string("input/four.txt").unwrap();

    let valid_passports_without_rules = input
        .split("\n\n")
        .map(|passport| passport.replace('\n', " "))
        .filter(|passport| {
            let mut byr = false;
            let mut iyr = false;
            let mut eyr = false;
            let mut hgt = false;
            let mut hcl = false;
            let mut ecl = false;
            let mut pid = false;
            passport
                .split(' ')
                .filter_map(|field| field.split_once(':'))
                .map(|(field, _)| field)
                .for_each(|field| match field {
                    "byr" => byr = true,
                    "iyr" => iyr = true,
                    "eyr" => eyr = true,
                    "hgt" => hgt = true,
                    "hcl" => hcl = true,
                    "ecl" => ecl = true,
                    "pid" => pid = true,
                    "cid" => {}
                    _ => panic!("Invalid field!"),
                });
            byr && iyr && eyr && hgt && hcl && ecl && pid
        })
        .count();
    println!(
        "valid_passports_without_rules: {}",
        valid_passports_without_rules
    );

    let valid_passports_with_rules = input
        .split("\n\n")
        .map(|passport| passport.replace('\n', " "))
        .filter(|passport| {
            let mut byr = false;
            let mut iyr = false;
            let mut eyr = false;
            let mut hgt = false;
            let mut hcl = false;
            let mut ecl = false;
            let mut pid = false;
            passport
                .split(' ')
                .filter_map(|field| field.split_once(':'))
                .for_each(|(field, data)| match field {
                    "byr" => {
                        if let Ok(num) = data.parse::<usize>() {
                            byr = num >= 1920 && num <= 2002;
                        }
                    }
                    "iyr" => {
                        if let Ok(num) = data.parse::<usize>() {
                            iyr = num >= 2010 && num <= 2020;
                        }
                    }
                    "eyr" => {
                        if let Ok(num) = data.parse::<usize>() {
                            eyr = num >= 2020 && num <= 2030;
                        }
                    }
                    "hgt" => {
                        if data.ends_with("cm") {
                            if let Ok(num) = data.replace("cm", "").parse::<usize>() {
                                hgt = num >= 150 && num <= 193;
                            }
                        } else if data.ends_with("in") {
                            if let Ok(num) = data.replace("in", "").parse::<usize>() {
                                hgt = num >= 59 && num <= 76;
                            }
                        }
                    }
                    "hcl" => {
                        if data.starts_with("#") && data.chars().count() == 7 {
                            hcl = data
                                .chars()
                                .skip(1)
                                .filter(|c| !c.is_ascii_hexdigit())
                                .count()
                                == 0
                        }
                    }
                    "ecl" => {
                        ecl = data == "amb"
                            || data == "blu"
                            || data == "brn"
                            || data == "gry"
                            || data == "grn"
                            || data == "hzl"
                            || data == "oth"
                    }
                    "pid" => {
                        if data.chars().count() == 9 {
                            pid = data.parse::<usize>().is_ok()
                        }
                    }
                    "cid" => {}
                    _ => panic!("Invalid field!"),
                });
            byr && iyr && eyr && hgt && hcl && ecl && pid
        })
        .count();
    println!(
        "valid_passports_with_rules   : {}",
        valid_passports_with_rules
    );
}
