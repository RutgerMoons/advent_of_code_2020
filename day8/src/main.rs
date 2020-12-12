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

#[derive(Debug, PartialEq)]
enum ConsoleState {
    Running,
    InfiniteLoop,
    Finished,
}

#[derive(Debug)]
struct Console {
    acc : i32,
    ip  : usize, // instruction pointer
    consoleState : ConsoleState,
}

impl Default for Console {
    fn default() -> Self {
        Console {
            acc : 0,
            ip : 0,
            consoleState : ConsoleState::Running,
        }
    }
}

impl Console {
    fn simulate_step(&mut self, instr : &Op) {
        match instr {
            Op::Acc(x) => {
                self.acc += x;
                self.ip += 1;
            },
            Op::Jmp(x) => {
                let jump = x + (self.ip as i32);
                self.ip = jump as usize;
            }
            Op::Nop(_) => {
                self.ip += 1;
            }
        }
    }

    fn simulate_program(&mut self, program : &Vec<Op>) {
        let mut visited: HashSet<usize> = HashSet::new();

        while self.consoleState == ConsoleState::Running {
            let instr = &program[self.ip];
            visited.insert(self.ip);
            self.simulate_step(instr);

            if self.ip >= program.len() {
                self.consoleState = ConsoleState::Finished;
            }

            if visited.contains(&self.ip) {
                self.consoleState = ConsoleState::InfiniteLoop;
            }
        }
    }
}

fn solve_part_1(mut cons : Console, program: &Vec<Op>) -> i32 {
    cons.simulate_program(program);
    cons.acc
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
        .. Default::default()
    };

    let result = solve_part_1(cons, &instructions);
    println!("Part 1: {}", result);

    Ok(())
}
