use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;
use std::ops::RangeInclusive;

type Ticket = Vec<usize>;
type Test = RangeInclusive<usize>;

fn solve_part_1(ranges: &Vec<Test>, nearby: &Vec<Ticket>) -> usize {
    let mut invalid: Vec<usize> = Vec::new();

    for near in nearby {
        for val in near {
            let mut valid: bool = false;
            for r in ranges {
                if r.contains(val) {
                    valid = true;
                }
            }
            if !valid {
                invalid.push(*val);
            }
        }
    }
    invalid.iter().fold(0, |acc, val| acc + val)
}

fn main() -> io::Result<()> {
    let file = File::open("/home/rutger/old_home/arch-rutger/Programming/advent_of_code_2020/input/day16.txt")?;
    let reader = BufReader::new(file);

    let lines : Vec<String> = reader.lines()
                                .filter_map(|line| line.ok())
                                .collect();

    let re_range = Regex::new(r"^(.*): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
    let re_ticket = Regex::new(r"(\d+)").unwrap();

    let mut ranges: Vec<Test> = Vec::new();
    let mut ticket: Ticket = Vec::new();
    let mut nearby: Vec<Ticket> = Vec::new();
    let mut parse_phase = 0;

    for line in lines {
        if line == "" { continue; }
        if line == "your ticket:" {
            parse_phase = 1;
            continue;
        }
        if line == "nearby tickets:" {
            parse_phase = 2;
            continue;
        }

        if parse_phase == 0 && re_range.is_match(&line) {
            for cap in re_range.captures_iter(&line) {
                let r1b = cap[2].parse().unwrap();
                let r1e = cap[3].parse().unwrap();
                let r2b = cap[4].parse().unwrap();
                let r2e = cap[5].parse().unwrap();
                ranges.push(r1b..=r1e);
                ranges.push(r2b..=r2e);
            }
        }

        if parse_phase == 2 && re_ticket.is_match(&line) {
            let mut n_ticket: Ticket = Vec::new();
            for cap in re_ticket.captures_iter(&line) {
                let nr: usize = cap[1].parse().unwrap();
                n_ticket.push(nr);
            }
            nearby.push(n_ticket);
        }
    }

    let result = solve_part_1(&ranges, &nearby);
    println!("Result of part 1: {}", result);

    /*
    let result = solve_part_2(&instructions);
    println!("Result of part 2: {}", result);
    */

    Ok(())
}

