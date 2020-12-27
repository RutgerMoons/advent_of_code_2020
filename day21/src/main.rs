use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

type Food = (HashSet<String>, Vec<String>);
type AllMap = HashMap<String, HashSet<String>>;

fn get_allergen_map(foods: &Vec<Food>) -> AllMap {
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
    allergen_map
}

fn solve_part_1(foods: &Vec<Food>) -> usize {
    let allergen_map = get_allergen_map(foods);

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

fn solve_part_2(foods: &Vec<Food>) -> String {
    let mut allergen_map = get_allergen_map(foods);

    let mut ing_map: HashMap<String, String> = HashMap::with_capacity(allergen_map.len());
    for (allergen, ingredients) in allergen_map.clone().iter() {
        if ingredients.len() == 1 {
            ing_map.insert(ingredients.iter().cloned().take(1).next().unwrap(), allergen.to_string());
            allergen_map.remove(allergen);
        }
    }
    let mut complete: bool = false;

    while ! complete {
        complete = true;
        for (allergen, ingredients) in allergen_map.clone().iter() {
            let ing_list: Vec<String> = ingredients.iter().cloned().collect();
            for ing in ing_list {
                if ing_map.contains_key(&ing) {
                    allergen_map.get_mut(allergen).unwrap().remove(&ing);
                }
            }

            let good = ingredients.len() == 1;
            if good {
                ing_map.insert(ingredients.iter().cloned().take(1).next().unwrap(), allergen.to_string());
                allergen_map.remove(allergen);
            } else {
                complete = false;
            }
        }
    }

    let mut sorted: Vec<(&str, &str)> = ing_map.iter().map(|(x, y)| (y.as_str(), x.as_str())).collect();
    sorted.sort();

    let mut result: String = String::new();
    for (_allergen, ing) in sorted {
        if result.len() > 0 {
            result.push(',');
        }
        result.push_str(ing);
    }

    result
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

    let result = solve_part_2(&foods);
    println!("Result of part 2 is {}", result);

    Ok(())
}
