use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;
use std::collections::VecDeque;

type Deck = VecDeque<u32>;

fn solve_part_1(d1: &mut Deck, d2: &mut Deck) -> u32 {
    let mut l1 = d1.len();
    let mut l2 = d2.len();

    while l1 > 0 && l2 > 0 {
        let c1 = d1.pop_front().unwrap();
        let c2 = d2.pop_front().unwrap();

        if c1 > c2 {
            d1.push_back(c1);
            d1.push_back(c2);
            l1 += 1;
            l2 -= 1;
        }
        else {
            d2.push_back(c2);
            d2.push_back(c1);
            l1 -= 1;
            l2 += 1;
        }
    }

    d1.iter()
      .rev()
      .enumerate()
      .fold(0, |acc, (idx, num)| acc + (idx as u32 + 1) * num)
    +
    d2.iter()
      .rev()
      .enumerate()
      .fold(0, |acc, (idx, num)| acc + (idx as u32 + 1) * num)
}

fn main() -> io::Result<()> {
    let file = File::open("/home/rutger/old_home/arch-rutger/Programming/advent_of_code_2020/input/day22.txt")?;
    let reader = BufReader::new(file);

    let mut deck1: Deck = VecDeque::new();
    let mut deck2: Deck = VecDeque::new();
    let re_player: Regex = Regex::new(r"^Player (\d+):$").unwrap();
    let mut cur_player = 0;
    for l in reader.lines() {
        let line = l?;
        if line.as_str() == "" { continue; }

        if re_player.is_match(&line) {
            for cap in re_player.captures_iter(&line) {
                cur_player = cap[1].parse().unwrap();
            }
            continue;
        }

        let num = line.parse().unwrap();
        if cur_player == 1 { deck1.push_back(num); }
        if cur_player == 2 { deck2.push_back(num); }
    }

    let result = solve_part_1(&mut deck1, &mut deck2);
    println!("Result of part 1: {}", result);

    Ok(())
}
