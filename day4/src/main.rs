use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;

fn solve(passports : &Vec<String>) -> u32 {
    let mut nb_valid = 0;
    for pass in passports {
        let mut byr = 0;
        let re_byr = Regex::new(r"byr:(\d{4})").unwrap();
        for cap in re_byr.captures_iter(pass) {
            byr = cap[1].parse().unwrap();
        }
        if byr < 1920 || byr > 2002 { continue ; }

        let mut iyr = 0;
        let re_iyr = Regex::new(r"iyr:(\d{4})").unwrap();
        for cap in re_iyr.captures_iter(pass) {
            iyr = cap[1].parse().unwrap();
        }
        if iyr < 2010 || iyr > 2020 { continue ; }

        let mut eyr = 0;
        let re_eyr = Regex::new(r"eyr:(\d{4})").unwrap();
        for cap in re_eyr.captures_iter(pass) {
            eyr = cap[1].parse().unwrap();
        }
        if eyr < 2020 || eyr > 2030 { continue ; }


        let mut hgt = 0;
        let mut hgt_unit = "".to_owned();
        let re_hgt = Regex::new(r"hgt:(\d{2,3})(\w{2})").unwrap();
        for cap in re_hgt.captures_iter(pass) {
            hgt = cap[1].parse().unwrap();
            hgt_unit = cap[2].to_string();
        }
        match hgt_unit.as_str() {
            "cm" => if hgt < 150 || hgt > 193 { continue ; },
            "in" => if hgt < 59 || hgt > 76 { continue ; },
            _ => { continue ; },
        }
        
        let mut hcl = "".to_owned();
        let re_hcl = Regex::new(r"hcl:(#[0-9a-f]{6})").unwrap();
        for cap in re_hcl.captures_iter(pass) {
            hcl = cap[1].to_string();
        }
        if hcl == "" { continue ; }

        let mut ecl = "".to_owned();
        let re_ecl = Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)").unwrap();
        for cap in re_ecl.captures_iter(pass) {
            ecl = cap[1].to_string();
        }
        if ecl == "" { continue ; }

        let mut pid = "".to_owned();
        let re_pid = Regex::new(r"pid:(\d*)").unwrap();
        for cap in re_pid.captures_iter(pass) {
            pid = cap[1].to_string();
        }
        let leng = pid.chars().count();
        if pid == "" || leng != 9 { continue ; }

        nb_valid += 1;
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
    println!("Result of part 2 is {}", result);

    /*
    let result = solve_part_2(&landscape);
    println!("Result of part 2 is {}", result);
    */

    Ok(())
}
