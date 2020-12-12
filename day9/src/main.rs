use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::VecDeque;

fn solve_part_1(nrs : &Vec<u64>, preamble_size : usize) -> u64 {
    let mut queue : VecDeque<u64> = VecDeque::new();
    for n in nrs[0..preamble_size].to_vec() {
        queue.push_back(n);
    }
    
    let mut work_num = 0;
    let mut idx = preamble_size;
    while idx < nrs.len() {
        work_num = nrs[idx];
        let mut found = false;

        for n in queue.iter() {
            if *n > work_num { continue ;}
            let n2 = work_num - n;

            if n2 == *n { continue; }
            if queue.contains(&n2) {
                found = true;
                break;
            }
        }

        if !found {
            break;
        }
        idx += 1;
        queue.pop_front();
        queue.push_back(work_num);
    }
    
    work_num
}

fn solve_part_2(nrs : &Vec<u64>, bad : u64) -> u64 {
    let mut queue : VecDeque<u64> = VecDeque::new();
    let mut sum = 0;

    for i in 0..nrs.len() {
        let n = nrs[i];
        sum = sum + n;
        queue.push_back(n);

        if sum == bad {
            return queue.iter().min().unwrap() + queue.iter().max().unwrap();
        }

        while sum > bad {
            if let Some(front) = queue.pop_front() {
                sum -= front;
            }
        }
    }

    0
}

fn main() -> io::Result<()> {
    let file = File::open("/home/rutger/old_home/arch-rutger/Programming/advent_of_code_2020/input/day9.txt")?;
    let reader = BufReader::new(file);

    let numbers : Vec<u64> = reader.lines()
                                   .filter_map(|line| line.ok())
                                   .map(|line| line.parse().unwrap())
                                   .collect();

    let result = solve_part_1(&numbers, 25);
    println!("Result of part 1: {}", result);

    let result2 = solve_part_2(&numbers, result);
    println!("Result of part 2: {}", result2);

    Ok(())
}
