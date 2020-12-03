use std::fs::File;
use std::io::{self, prelude::*, BufReader};
struct PasswordPolicy {
    min : u32,
    max : u32,
    letter : String
}

struct PassLine {
    pol : PasswordPolicy,
    pass : String
}

fn solve_part_1(lines : &Vec<PassLine>) -> u32 {
    let mut cnt = 0;
    for line in lines {
        let pol = &line.pol;
        let ch : &char = &pol.letter.chars().next().unwrap();
        let nb = line.pass.chars().filter(|c| c == ch).count() as u32;
        if nb >= pol.min && nb <= pol.max {
            cnt += 1;
        }
    }
    cnt
}

fn main() -> io::Result<()> {
    let file = File::open("/home/rutger/old_home/arch-rutger/Programming/advent_of_code_2020/input/day2_1.txt")?;
    let reader = BufReader::new(file);
    let mut passlines : Vec<PassLine> = Vec::new();

    for line in reader.lines() {
        let l = line?;
        let spl : Vec<&str> = l.split(":").collect();
        let spl2 : Vec<&str> = spl[0].split(" ").collect();

        let (minmaxstr, letter) = (spl2[0], spl2[1]);
        let spl3 : Vec<&str> = minmaxstr.split("-").collect();
        let (minstr, maxstr) = (spl3[0], spl3[1]);

        let min = match minstr.parse() {
            Ok(m) => m,
            Err(_) => 9999999,
        };
        let max = match maxstr.parse() {
            Ok(m) => m,
            Err(_) => 9999999,
        };

        let policy = PasswordPolicy {min : min, max : max, letter : letter.to_string() };
        let passline = PassLine { pol: policy, pass : spl[1].to_string() };

        passlines.push(passline);
    }

    let result = solve_part_1(&passlines);
    println!("Result part 1: {}", result);

    Ok(())
}
