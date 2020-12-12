use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug,Clone,Copy)]
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
    console_state : ConsoleState,
}

impl Default for Console {
    fn default() -> Self {
        Console {
            acc : 0,
            ip : 0,
            console_state : ConsoleState::Running,
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

        while self.console_state == ConsoleState::Running {
            let instr = &program[self.ip];
            visited.insert(self.ip);
            self.simulate_step(instr);

            if self.ip >= program.len() {
                self.console_state = ConsoleState::Finished;
            }

            if visited.contains(&self.ip) {
                self.console_state = ConsoleState::InfiniteLoop;
            }
        }
    }
}

fn solve_part_1(program: &Vec<Op>) -> i32 {
    let mut cons = Console { .. Default::default() };
    cons.simulate_program(program);
    cons.acc
}

fn solve_part_2( orig_program: &Vec<Op>) -> i32 {
    let mut work_program : Vec<Op> = orig_program.to_vec();
    loop {
        for i in 0..orig_program.len() {
            let instr = work_program[i];
            match instr {
                Op::Acc(_) => continue,
                Op::Jmp(x) => work_program[i] = Op::Nop(x),
                Op::Nop(x) => work_program[i] = Op::Jmp(x),
            }
            let mut cons = Console::default();
            cons.simulate_program(&work_program);
            if cons.console_state == ConsoleState::Finished {
                return cons.acc;
            } else {
                work_program[i] = instr;
            }
        }
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

    let result = solve_part_1(&instructions);
    println!("Part 1: {}", result);

    let result = solve_part_2(&instructions);
    println!("Part 2: {}", result);

    Ok(())
}
