use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn solve(nrs: &Vec<u32>) -> Option<u32> {
    for i in nrs {
        if nrs.contains(&(2020 - i)) {
            return Some(i * (2020 - i));
        }
    }
    None
} 

fn main() -> io::Result<()> {
    let file = File::open("/home/rutger/old_home/arch-rutger/Programming/advent_of_code_2020/input/day1_1.txt")?;
    let reader = BufReader::new(file);
    let mut nrs : Vec<u32> = Vec::new();

    for line in reader.lines() {
        let x = match line?.parse() {
            Ok(x) => x,
            Err(_) => 0
        };

        if x > 0 {
            nrs.push(x);
        }
    }

    if let Some(result) = solve(&nrs) {
        println!("{}", result);
    }
    Ok(())
}
