use std::fs::File;
use std::io::{self, prelude::*, BufReader};

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

fn main() -> io::Result<()> {
    let file = File::open("/home/rutger/old_home/arch-rutger/Programming/advent_of_code_2020/input/day6.txt")?;
    let reader = BufReader::new(file);
    let mut answers : Vec<String> = Vec::new();

    let mut answer : String = "".to_owned();
    for line in reader.lines() {
        let l = line?;
        if l == "" {
            answers.push(answer);
            answer = "".to_owned();
            continue;
        }

        answer = answer + " " + &l;
    }

    let result = solve(&answers);
    println!("Result of part 1 is {}", result);

    //let result = solve_part_2(&passports);
    //println!("Result of part 2 is {}", result);

    Ok(())
}