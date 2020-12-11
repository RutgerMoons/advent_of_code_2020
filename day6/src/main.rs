use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;

fn solve(answers: &Vec<String>) -> u32 {
    let mut result = 0;
    for ans in answers {
        let mut letters_list : Vec<char> = ans.chars().filter(|l| l.is_alphabetic()).collect();
        letters_list.sort();
        
        let mut prev_letter = '!';
        for letter in letters_list {
            if prev_letter != letter {
                prev_letter = letter;
                result += 1;
            }        
        }
    }
    result
}

fn solve_part_2(sets: &Vec<HashSet<char>>) -> u32 {
    let mut result : u32 = 0;
    if let Some(s) = sets.get(0) {
        for c in s {
            let mut cont : bool = true;
            for o_set in sets {
                if !o_set.contains(c) {
                    cont = false;
                    break;
                }
            }
            if cont {
                result += 1;
            }
        }
    }
    result
}

fn main() -> io::Result<()> {
    let file = File::open("/home/rutger/old_home/arch-rutger/Programming/advent_of_code_2020/input/day6.txt")?;
    let reader = BufReader::new(file);
    let mut answers : Vec<String> = Vec::new();

    let mut answer : String = "".to_owned();
    let mut answer_sets: Vec<HashSet<char>> = Vec::new();
    let mut result2 : u32 = 0;
    for line in reader.lines() {
        let l = line?;
        if l == "" {
            answers.push(answer);
            answer = "".to_owned();

            result2 += solve_part_2(&answer_sets);
            answer_sets = Vec::new();
            continue;
        }

        answer = answer + " " + &l;

        let mut s = HashSet::new();
        for c in l.chars().filter(|l| l.is_alphabetic()) {
            s.insert(c);
        }
        answer_sets.push(s);
    }

    let result = solve(&answers);
    println!("Result of part 1 is {}", result);

    println!("Result of part 2 is {}", result2);

    Ok(())
}