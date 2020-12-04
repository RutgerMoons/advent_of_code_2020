use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn solve_part_2(landscape: &Vec<String>) -> u64 {
    let mut total = 1;
    let x = solve(landscape, 1, 1);
    println!("a {} {}", x, total);
    total *= x;

    let x = solve(landscape, 1, 3);
    println!("b {} {}", x, total);
    total *= x;

    let x = solve(landscape, 1, 5);
    println!("c {} {}", x, total);
    total *= x;

    let x = solve(landscape, 1, 7);
    println!("d {} {}", x, total);
    total *= x;

    let x = solve(landscape, 2, 1);
    println!("e {} {}", x, total);
    total * x
}

fn solve(landscape: &Vec<String>, steps_down: usize, steps_right: usize) -> u64 {
    let mut col : usize = 0;
    let mut nb_trees = 0;
    let mut line_length = 1;

    for (idx, line) in landscape.iter().enumerate() {
        if idx == 0 {
            line_length = line.chars().count();
            continue;
        }

        if idx % steps_down != 0 { continue; }

        col += steps_right;
        col %= line_length;

        if line.chars().nth(col).unwrap() == '#' {
            nb_trees += 1;
        }
    }
    nb_trees
}

fn main() -> io::Result<()> {
    let file = File::open("/home/rutger/old_home/arch-rutger/Programming/advent_of_code_2020/input/day3.txt")?;
    let reader = BufReader::new(file);
    let mut landscape : Vec<String> = Vec::new();

    for line in reader.lines() {
        let l = line?;
        landscape.push(l);
    }

    let result = solve(&landscape, 1, 3);
    println!("Result of part 1 is {}", result);

    let result = solve_part_2(&landscape);
    println!("Result of part 2 is {}", result);

    Ok(())
}
