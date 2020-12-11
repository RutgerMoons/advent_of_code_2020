use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug)]
enum Op {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

#[derive(Debug)]
struct Console {
    acc : i32,
    ip  : usize, // instruction pointer
    instructions: Vec<Op>,
}

impl Default for Console {
    fn default() -> Self {
        Console {
            acc : 0,
            ip : 0,
            instructions: Vec::new(),
        }
    }
}

impl Console {
    fn simulate(&mut self) {
        let instr = &self.instructions[self.ip];
        match instr {
            Op::Acc(x) => {
                self.acc += x;
                self.ip += 1;
            },
            Op::Jmp(x) => {
                let jump = *x + (self.ip as i32);
                self.ip = jump as usize;
            }
            Op::Nop(_) => {
                self.ip += 1;
            }
        }
    }
}

fn solve_part_1(mut cons : Console) -> i32 {
    let mut visited = HashSet::new();

    loop {
        if visited.contains(&cons.ip) {
            return cons.acc;
        }

        visited.insert(cons.ip);
        cons.simulate();
    }
}

impl From<(&str, i32)> for Op {
    fn from(input : (&str, i32)) -> Self {
        match input.0 {
            "acc" => Op::Acc(input.1),
            "nop" => Op::Nop(input.1),
            "jmp" => Op::Jmp(input.1),
            _ => panic!(),
        }
    }
}

fn main() -> io::Result<()> {
    let file = File::open("/home/rutger/old_home/arch-rutger/Programming/advent_of_code_2020/input/day8.txt")?;
    let reader = BufReader::new(file);

    let re = Regex::new(r"^(nop|acc|jmp) ((?:\+|-)(?:\d+))$").unwrap();
    let instructions : Vec<Op> = 
    reader.lines().map(|line| -> Op {
        for cap in re.captures_iter(&line.unwrap()) {
            let operation = &cap[1];
            let operand : i32 = cap[2].parse().unwrap();
            return (operation, operand).into();
        }
        panic!();
    }).collect();

    let cons = Console {
        instructions : instructions,
        .. Default::default()
    };

    let result = solve_part_1(cons);
    println!("Part 1: {}", result);

    Ok(())
}
