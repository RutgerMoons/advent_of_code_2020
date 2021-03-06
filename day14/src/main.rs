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

fn generate_floating_masks(mut floats: &mut Vec<usize>) -> Vec<u64> {
    let masks = vec![0];
    mask_helper(masks, &mut floats)
}

fn mask_helper(masks: Vec<u64>, floats: &mut Vec<usize>) -> Vec<u64> {
    let f = floats.pop();
    if f.is_none() {
        return masks
    }
    let f = f.unwrap();

    let mut new_masks = Vec::with_capacity(masks.len() * 2);
    for mask in masks {
        let set1 = mask | (1 << f);
        let set0 = mask;
        new_masks.push(set1);
        new_masks.push(set0);
    }

    mask_helper(new_masks, floats)
}

fn solve_part_2(instr: &Vec<Instruction>) -> i64 {
    let mut and_masks: Vec<u64> = Vec::new();
    let mut or_mask: u64 = 0;
    let mut and_mask: u64 = 0;
    let mut mem: HashMap<u64, i64> = HashMap::new();
    let mut inst_idx = 0;

    for inst in instr {
        inst_idx += 1;
        println!("Instruction {}", inst_idx);
        match inst {
            Instruction::Mask(mask_str) => {
                let mut x_loc: Vec<usize> = Vec::new();
                and_mask = 0;
                or_mask = 0;
                for (idx, c) in mask_str.chars().rev().enumerate() {
                    match c {
                        'X' => {
                            x_loc.push(idx);
                            and_mask = and_mask | (1 << idx);
                         } ,
                        '1' => {
                            or_mask = or_mask | (1 << idx);
                        },
                        '0' => {} , 
                        _ => unreachable!(), 
                    }    
                }
                and_masks = generate_floating_masks(&mut x_loc);
                and_mask = !and_mask;
            },
            Instruction::Mem(mem_idx, mem_val) => {
                let masked_mem: u64 = (*mem_idx | or_mask) & and_mask;
                //println!("or {:#b} and {:#b}, masked: {:#b}", &or_mask, &and_mask, &masked_mem);
                /*
                for i in (0..last_x).map(|x| x & and_mask) {
                    let masked = (masked_mem & i) | i;
                    mem.insert(masked, *mem_val);
                }
                */
                for mask in and_masks.iter() {
                    let masked = masked_mem | mask;
                    mem.insert(masked, *mem_val);
                }
            }
        }
    }

    mem.values().fold(0, |acc, x| {acc + x})
}

fn solve_part_1(instr: &Vec<Instruction>) -> i64 {
    run_instructions(instr)
}

fn main() -> io::Result<()> {
    let file = File::open("/home/rutger/old_home/arch-rutger/Programming/advent_of_code_2020/input/day14.txt")?;
    let reader = BufReader::new(file);

    let lines : Vec<String> = reader.lines()
                                .filter_map(|line| line.ok())
                                .collect();

    let re_mask = Regex::new(r"^mask = ([X|1|0]+)$").unwrap();
    let re_mem = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();

    let mut instructions: Vec<Instruction> = Vec::new();

    for line in lines {
        if re_mask.is_match(&line) {
            for cap in re_mask.captures_iter(&line) {
                instructions.push(Instruction::Mask(cap[1].to_string()));
            }
        }
        else if re_mem.is_match(&line) {
            for cap in re_mem.captures_iter(&line) {
                let mem_idx: u64 = cap[1].parse().unwrap();
                let mem_val: i64 = cap[2].parse().unwrap();
                instructions.push(Instruction::Mem(mem_idx, mem_val));
            }
        }
    }

    let result = solve_part_1(&instructions);
    println!("Result of part 1: {}", result);

    let result = solve_part_2(&instructions);
    println!("Result of part 2: {}", result);

    Ok(())
}
