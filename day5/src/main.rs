use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn solve(boarding_list : &Vec<String>) -> u32 {
    if let Some(x) = boarding_list.iter().map(|pass| solve_id(&pass)).max() {
        return x
    }
    0
}

fn solve_row_col(pass: &str) -> (u32, u32) {
    let mut min_row: u32 = 0;
    let mut max_row: u32 = 127;
    let mut min_col: u32 = 0;
    let mut max_col: u32 = 7;

    for char in pass.chars() {
        match char {
            'B' => min_row += (max_row - min_row) / 2 + 1,
            'F' => max_row -= (max_row - min_row) / 2 + 1,
            'L' => max_col -= (max_col - min_col) / 2 + 1,
            'R' => min_col += (max_col - min_col) / 2 + 1, 
            _ => unreachable!(),
        }
    }
    (min_row, min_col)
}

fn solve_id(pass : &str) -> u32 {
    let (min_row, min_col) = solve_row_col(pass);
    min_row * 8 + min_col
}

fn solve_part_2(boarding_passes: &Vec<String>) -> u32 {
    let mut ids : Vec<u32> = boarding_passes.iter()
                                        .map(|pass| solve_id(&pass))
                                        .collect();
    ids.sort();
    for ch in ids.windows(2) {
        if ch[1] - ch[0] == 2 {
            return ch[0] + 1;
        }
    }
    0
}

fn main() -> io::Result<()> {
    let file = File::open("/home/rutger/old_home/arch-rutger/Programming/advent_of_code_2020/input/day5.txt")?;
    let reader = BufReader::new(file);
    let mut boarding_passes : Vec<String> = Vec::new();

    for line in reader.lines() {
        let l = line?;
        boarding_passes.push(l);
    }

    let result = solve(&boarding_passes);
    println!("Result of part 1 is {}", result);

    let result = solve_part_2(&boarding_passes);
    println!("Result of part 2 is {}", result);

    Ok(())
}