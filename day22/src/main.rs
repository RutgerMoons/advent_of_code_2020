use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;
use std::collections::VecDeque;
use std::collections::HashSet;

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

fn solve_part_2(d1: &mut Deck, d2: &mut Deck) -> u32 {
    let mut l1 = d1.len() as u32;
    let mut l2 = d2.len() as u32;
    let mut seen_pos: HashSet<Deck> = HashSet::new();

    while l1 > 0 && l2 > 0 {
        // check recursion
        if seen_pos.contains(&d1) {
            return d1.iter() .rev() .enumerate() .fold(0, |acc, (idx, num)| acc + (idx as u32 + 1) * num);
        }
        seen_pos.insert(d1.clone());

        let c1 = d1.pop_front().unwrap();
        let c2 = d2.pop_front().unwrap();

        // at least c1 and c2 cards in both decks -> Recurse
        let winner = if l1 > c1 && l2 > c2 {
            decide_winner(d1.clone().iter().cloned().take(c1 as usize).collect(), d2.clone().iter().cloned().take(c2 as usize).collect())
        } else {
            if c1 > c2 { 1 } 
            else { 2 }
        };

        if winner == 1 {
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

fn decide_winner(mut d1: Deck, mut d2: Deck) -> u32 {
    let mut l1 = d1.len() as u32;
    let mut l2 = d2.len() as u32;
    let mut seen_pos: HashSet<Deck> = HashSet::new();

    while l1 > 0 && l2 > 0 {
        // check recursion
        if seen_pos.contains(&d1) {
            return 1;
        }
        seen_pos.insert(d1.clone());

        let c1 = d1.pop_front().unwrap();
        let c2 = d2.pop_front().unwrap();

        // at least c1 and c2 cards in both decks -> Recurse
        let winner = if l1 > c1 && l2 > c2 {
            decide_winner(d1.clone().iter().cloned().take(c1 as usize).collect(), d2.clone().iter().cloned().take(c2 as usize).collect())
        } else if c1 > c2 { 1
        } else { 2 };

        if winner == 1 {
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

    if l1 == 0 {
        2
    } else {
        1
    }
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

    let result = solve_part_1(&mut deck1.clone(), &mut deck2.clone());
    println!("Result of part 1: {}", result);

    let result = solve_part_2(&mut deck1.clone(), &mut deck2.clone());
    println!("Result of part 2: {}", result);

    Ok(())
}
