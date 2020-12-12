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

fn get_nb_paths(mut start : u64, middle : &mut Vec<u64>, end : u64) -> u64 {
    if middle.len() == 0 {
        return 1;
    }

    // not the first round
    if start != 0 {
        start = middle.remove(0);
    }

    if end - start <= 3 {
        let l = middle.len() as u32;
        if l < 1 {
            return 1;
        } else {
            return 2_u64.pow(l);
        }
    }

    // try_paths
    permute_paths(start, middle, end)
}

fn permute_paths(start : u64, middle : &Vec<u64>, end : u64) -> u64 {
    let mut total = 0;
    let l = middle.len();

    for i in 1..2_u64.pow(l as u32) {
        let mut good = true;
        let mut prev = start;

        for j in 0..l {
            if i & (i << j) != 0 {
                let curr = middle[j];
                if curr - prev > 3 {
                    good = false;
                    break;
                } else {
                    prev = curr;
                }
            }
        }

        

        if good && end - prev <= 3 {
            total += 1;
        }
    }
    total
}

fn solve_part_2(adapters : &Vec<u64>) -> u64 {
    let mut paths = 1;
    let mut ad : Vec<u64> = Vec::new();

    let mut prev : u64 = 0;
    for w in adapters.windows(2) {
        if w[1] - w[0] == 3 {
            paths *= get_nb_paths(prev, &mut ad, w[0]);
            prev = w[0];
            ad = Vec::new();
        } else {
            ad.push(w[0]);
        }
    }

    let last = adapters[adapters.len() - 1];
    paths *= get_nb_paths(prev, &mut ad, last);
    paths
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

    let result = solve_part_2(&numbers,);
    println!("Result of part 2: {}", result);

    Ok(())
}