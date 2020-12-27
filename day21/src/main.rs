use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

type Food = (HashSet<String>, Vec<String>);
type AllMap = HashMap<String, HashSet<String>>;

fn solve_part_1(foods: &Vec<Food>) -> usize {
    let mut allergen_map: AllMap = HashMap::new();
    for (ings, all) in foods.iter().cloned() {
        for allergen in all {
            match allergen_map.get(&allergen) {
                Some(set) => {
                    let intersection = set.intersection(&ings).cloned().collect();
                    allergen_map.insert(allergen, intersection);
                },
                None => {
                    allergen_map.insert(allergen, ings.clone());
                },
            }
        }
    }

    // reduce to candidates only, now find the none-candidates
    let mut cand_ings: HashSet<String> = HashSet::new();
    for cand in allergen_map.values() {
        for can in cand.iter().cloned() {
            cand_ings.insert(can);
        }
    }

    let mut counter = 0;
    for (ings, _) in foods.iter() {
        for ing in ings.iter() {
            if !cand_ings.contains(ing) {
                counter += 1;
            }
        }
    }

    counter
}

fn main() -> io::Result<()> {
    let file = File::open("/home/rutger/old_home/arch-rutger/Programming/advent_of_code_2020/input/day21.txt")?;
    let reader = BufReader::new(file);

    let mut foods: Vec<Food> = Vec::new();
    let re_split = Regex::new(r"^(.*)\(contains (.*)\)$").unwrap();

    for l in reader.lines() {
        let line = l?;
        for cap in re_split.captures_iter(&line) {
            let ing: HashSet<String> = cap[1].split_ascii_whitespace().map(|s| s.to_string()).collect();
            let allergens: Vec<String> = cap[2].split(", ").map(|s| s.to_string()).collect();
            foods.push((ing, allergens));
        }
    }
    
    let result = solve_part_1(&foods);
    println!("Result of part 1 is {}", result);

    Ok(())
}
