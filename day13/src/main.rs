use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn solve_part_1(start: u64, busses: &Vec<u64>) -> u64 {
    let min: (u64, u64) = 
        busses.iter()
            .map(|bus| (start + bus - (start % bus), *bus))
            .min()
            .unwrap();

    let wait_time = min.0 - start;
    let bus = min.1;
    wait_time * bus
}

fn main() -> io::Result<()> {
    let file = File::open("/home/rutger/old_home/arch-rutger/Programming/advent_of_code_2020/input/day13.txt")?;
    let reader = BufReader::new(file);

    let lines : Vec<String> = reader.lines()
                                .filter_map(|line| line.ok())
                                .collect();

    let start: u64 = lines[0].parse().unwrap();
    let busses: Vec<u64> = lines[1].split(",")
                                   .map(|x| x.parse())
                                   .filter_map(|n| n.ok())
                                   .collect();

    let result = solve_part_1(start, &busses);
    println!("Result of part 1: {}", result);

    Ok(())
}

