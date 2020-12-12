use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;

fn solve_part_1(adapters : &Vec<u64>) -> u64 {
    let mut diffs = HashMap::new();
    for w in adapters.windows(2) {
        let d = w[1] - w[0];
        if d >= 1 && d <= 3 {
            *diffs.entry(d).or_insert(0) += 1;
        }
    }

    if let Some(x) = adapters.get(0) {
        *diffs.entry(*x).or_insert(0) += 1;
    }

    diffs.get(&1).or(Some(&0)).unwrap() * 
    (diffs.get(&3).or(Some(&0)).unwrap() + 1)
}

fn main() -> io::Result<()> {
    let file = File::open("/home/rutger/old_home/arch-rutger/Programming/advent_of_code_2020/input/day10.txt")?;
    let reader = BufReader::new(file);

    let mut numbers : Vec<u64> = reader.lines()
                                   .filter_map(|line| line.ok())
                                   .map(|line| line.parse().unwrap())
                                   .collect();
    numbers.sort();

    let result = solve_part_1(&numbers);
    println!("Result of part 1: {}", result);

    /*
    let result2 = solve_part_2(&numbers, result);
    println!("Result of part 2: {}", result2);
    */

    Ok(())
}