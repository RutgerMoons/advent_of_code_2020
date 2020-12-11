use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
struct BagContent {
    amount : u32,
    color : String,
}

fn solve_part_1(map : &HashMap<String, Vec<String>>, key: &str) -> u32 {
    let mut colors = HashSet::new();
    fill_set(&mut colors, map, key);
    colors.len() as u32
}

fn fill_set(mut colors : &mut HashSet<String>, map : &HashMap<String, Vec<String>>, key: &str) {
    if let Some(outers) = map.get(key) {
        for bag in outers {
            fill_set(&mut colors, map, bag);
            colors.insert(bag.to_string());
        }
    }
}

fn solve_part_2(map : &HashMap<String, Vec<BagContent>>, key: &str) -> u32 {
    let mut total = 0;
    if let Some(contents) = map.get(key) {
        for bag_cont in contents {
            total += bag_cont.amount * (1 + solve_part_2(map, &bag_cont.color));
        }
    }
    total
}

fn main() -> io::Result<()> {
    let file = File::open("/home/rutger/old_home/arch-rutger/Programming/advent_of_code_2020/input/day7.txt")?;
    let reader = BufReader::new(file);

    let re_begin = Regex::new(r"^(.*) bags contain (.*)$").unwrap();
    let re_bags = Regex::new(r"(\d+) (.*?) bag").unwrap();

    let mut map = HashMap::new();
    let mut map2 = HashMap::new();
    
    for line in reader.lines() {
        let l = line?;
        if l == "" { continue ; }

        for cap in re_begin.captures_iter(&l) {
            let head = &cap[1];
            let tail = &cap[2];

            for cap in re_bags.captures_iter(tail) {
                let amount = cap[1].parse().unwrap();
                let inner = &cap[2];
                let outer = head;
                map.entry(inner.to_string()).or_insert(Vec::new()).push(outer.to_string());
                map2.entry(outer.to_string()).or_insert(Vec::new()).push(BagContent { amount : amount, color : inner.to_string()});
            }
        }
    }

    let result = solve_part_1(&map, "shiny gold");
    println!("Part 1: {}", result);

    let result = solve_part_2(&map2, "shiny gold");
    println!("Part 2: {}", result);

    Ok(())
}
