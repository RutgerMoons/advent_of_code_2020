use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
enum Instruction {
    Mask(String),
    Mem(u64, i64),
}

fn run_instructions(instr: &Vec<Instruction>) -> i64 {
    let mut and_mask = 0;
    let mut or_mask = 0;
    let mut mem: HashMap<u64, i64> = HashMap::new();

    for inst in instr {
        match inst {
            Instruction::Mask(mask_str) => {
                and_mask = 0;
                or_mask = 0;
                for (idx, c) in mask_str.chars().rev().enumerate() {
                    match c {
                        'X' => and_mask = and_mask | (1 << idx) ,
                        '1' => {
                            or_mask = or_mask | (1 << idx);
                            and_mask = and_mask | (1 << idx)
                        },
                        '0' => and_mask = and_mask & !(1 << idx) , 
                        _ => unreachable!(), 
                    }    
                }
            },
            Instruction::Mem(mem_idx, mem_val) => {
                let masked_val: i64 = (mem_val | or_mask) & and_mask;
                mem.insert(*mem_idx, masked_val);
            }
        }
    }

    mem.values().fold(0, |acc, x| {acc + x})
}

fn solve_part_1(lines: &Vec<String>) -> i64 {
    let re_mask = Regex::new(r"^mask = ([X|1|0]+)$").unwrap();
    let re_mem = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();

    let mut instructions: Vec<Instruction> = Vec::new();

    for line in lines {
        if re_mask.is_match(line) {
            for cap in re_mask.captures_iter(line) {
                instructions.push(Instruction::Mask(cap[1].to_string()));
            }
        }
        else if re_mem.is_match(line) {
            for cap in re_mem.captures_iter(line) {
                let mem_idx: u64 = cap[1].parse().unwrap();
                let mem_val: i64 = cap[2].parse().unwrap();
                instructions.push(Instruction::Mem(mem_idx, mem_val));
            }
        }
    }

    run_instructions(&instructions)
}

fn main() -> io::Result<()> {
    let file = File::open("/home/rutger/old_home/arch-rutger/Programming/advent_of_code_2020/input/day14.txt")?;
    let reader = BufReader::new(file);

    let lines : Vec<String> = reader.lines()
                                .filter_map(|line| line.ok())
                                .collect();

    let result = solve_part_1(&lines);
    println!("Result of part 1: {}", result);

    Ok(())
}
