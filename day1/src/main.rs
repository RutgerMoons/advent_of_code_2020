use std::fs::File;
use std::io::{self, prelude::*, BufReader};

type ReceiptEntry = u32;
type Receipt = Vec<ReceiptEntry>;

fn solve(nrs: &Receipt) -> Option<ReceiptEntry> {
    for i in nrs {
        if nrs.contains(&(2020 - i)) {
            return Some(i * (2020 - i));
        }
    }
    None
} 

fn solve_part_2(nrs: &Receipt) -> Option<ReceiptEntry> {
    let l = nrs.len();
    for (i1, n1) in nrs[0..(l - 3)].iter().enumerate() {
        for (i2, n2) in nrs[(i1 + 1)..(l - 2)].iter().enumerate() {
            let part_sum = n1 + n2;
            if part_sum >= 2020 {
                continue;
            }

            if nrs[(i2 + 1)..].contains(&(2020 - n1 - n2)) {
                return Some(n1 * n2 * (2020 - n1 - n2));
            }
        }
    }
    None
}

fn main() -> io::Result<()> {
    let file = File::open("/home/rutger/old_home/arch-rutger/Programming/advent_of_code_2020/input/day1_1.txt")?;
    let reader = BufReader::new(file);
    let mut nrs : Receipt = Vec::new();

    for line in reader.lines() {
        let x = match line?.parse() {
            Ok(x) => x,
            Err(_) => 0
        };

        if x > 0 {
            nrs.push(x);
        }
    }

    if let Some(result) = solve(&nrs) {
        println!("result part 1: {}", result);
    }

    if let Some(result) = solve_part_2(&nrs) {
        println!("result part 2: {}", result);
    }

    Ok(())
}
