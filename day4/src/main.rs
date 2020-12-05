use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;

fn solve(passports: &Vec<String>) -> u32 {
    let mut nb_valid = 0;
    let re_byr = Regex::new(r"byr:").unwrap();
    let re_iyr = Regex::new(r"iyr:").unwrap();
    let re_eyr = Regex::new(r"eyr:").unwrap();
    let re_hgt = Regex::new(r"hgt:").unwrap();
    let re_hcl = Regex::new(r"hcl:").unwrap();
    let re_ecl = Regex::new(r"ecl:").unwrap();
    let re_pid = Regex::new(r"pid:").unwrap();

    for pass in passports {
        if re_byr.is_match(pass) &&
           re_iyr.is_match(pass) &&
           re_eyr.is_match(pass) &&
           re_hgt.is_match(pass) &&
           re_hcl.is_match(pass) &&
           re_ecl.is_match(pass) &&
           re_pid.is_match(pass) {
               nb_valid += 1;
           }
    }
    nb_valid
}

fn solve_part_2(passports : &Vec<String>) -> u32 {
    let mut nb_valid = 0;
    let re_byr = Regex::new(r"byr:(19[2-9][0-9]|200[0-2])\b").unwrap();
    let re_iyr = Regex::new(r"iyr:20(1[0-9]|20)\b").unwrap();
    let re_eyr = Regex::new(r"eyr:20(2[0-9]|30)\b").unwrap();
    let re_hgt = Regex::new(r"hgt:((59|6[0-9]|7[0-6])in|(1[5-8][0-9]|19[0-3])cm)\b").unwrap();
    let re_hcl = Regex::new(r"hcl:#[0-9a-f]{6}\b").unwrap();
    let re_ecl = Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)\b").unwrap();
    let re_pid = Regex::new(r"pid:\d{9}\b").unwrap();
    for pass in passports {
        if re_byr.is_match(pass) &&
           re_iyr.is_match(pass) &&
           re_eyr.is_match(pass) &&
           re_hgt.is_match(pass) &&
           re_hcl.is_match(pass) &&
           re_ecl.is_match(pass) &&
           re_pid.is_match(pass) {
               nb_valid += 1;
           }
    }
    nb_valid
}

fn main() -> io::Result<()> {
    let file = File::open("/home/rutger/old_home/arch-rutger/Programming/advent_of_code_2020/input/day4.txt")?;
    let reader = BufReader::new(file);
    let mut passports : Vec<String> = Vec::new();

    let mut pass : String = "".to_owned();
    for line in reader.lines() {
        let l = line?;
        if l == "" {
            passports.push(pass);
            pass = "".to_owned();
            continue;
        }

        pass = pass + " " + &l;
    }

    let result = solve(&passports);
    println!("Result of part 1 is {}", result);

    let result = solve_part_2(&passports);
    println!("Result of part 2 is {}", result);

    Ok(())
}
