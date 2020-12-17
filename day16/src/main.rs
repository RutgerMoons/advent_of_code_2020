use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;
use std::ops::RangeInclusive;
use std::collections::HashMap;

type Ticket = Vec<usize>;
type Test = RangeInclusive<usize>;

struct FieldTest {
    field: String,
    test1: Test,
    test2: Test,
}

impl FieldTest {
    fn test(&self, field: &usize) -> bool {
        self.test1.contains(field) || self.test2.contains(field) 
    }
}

fn solve_part_2(tests: &Vec<FieldTest>, nearby: &Vec<Ticket>, mine: Ticket) -> u64{
    let filtered: Vec<Ticket> = nearby.iter()
                                      .filter(|tick| get_invalid_field(&tests, &tick).is_none()) // only good tickets
                                      .map(|x| x.clone())                                        // own them
                                      .collect();
    let ticket_length = filtered[0].len();
    let mut fields: HashMap<String, Vec<usize>> = HashMap::new();
    for test in tests {
        for i in 0..ticket_length {
            let mut good = true;
            for tick in filtered.iter() {
                if !test.test(&tick[i]) {
                    good = false;
                    break;
                }
            }
            if good {
                fields.entry(test.field.to_string()).or_insert(Vec::new()).push(i);
            }
        }
    }

    // reduce fields to unique positions
    let mut final_fields: HashMap<String, usize> = HashMap::new();
    let mut test_length: usize = 0;
    while test_length < ticket_length {
        // find uniques
        let mut uniques: Vec<usize> = Vec::new();
        for (key, val) in fields.iter() {
            if val.len() == 1 {
                uniques.push(val[0]);
                final_fields.insert(key.to_string(), val[0]);
            }
        }

        for u in uniques.iter() {
            for (_key, val) in fields.iter_mut() {
                if val.len() > 1 {
                    if let Some(pos) = val.iter().position(|x| *x == *u) {
                        val.remove(pos);
                    }   
                }   
            }
        }
        
        test_length = uniques.len();
    }

    let final_idx: Vec<usize> = final_fields.iter()
          .filter(|(f, _idx)| f.len() > 9 && &f[0..9] == "departure")
          .map(|(_, idx)| *idx)
          .collect();

    final_idx.iter().fold(1, |acc, idx| acc * mine[*idx] as u64)
}

fn get_invalid_field(tests: &Vec<FieldTest>, ticket: &Ticket) -> Option<usize> {
    for val in ticket {
        let mut valid: bool = false;
        for t in tests {
            if t.test(val) {
                valid = true;
            }
        }
        if !valid {
            return Some(*val);
        }
    }
    None
}

fn solve_part_1(tests: &Vec<FieldTest>, nearby: &Vec<Ticket>) -> usize {
    let mut invalid: Vec<usize> = Vec::new();

    for near in nearby {
        if let Some(inval) = get_invalid_field(&tests, &near) {
            invalid.push(inval);
        }
    }
    invalid.iter().fold(0, |acc, val| acc + val)
}

fn main() -> io::Result<()> {
    let file = File::open("/home/rutger/old_home/arch-rutger/Programming/advent_of_code_2020/input/day16.txt")?;
    let reader = BufReader::new(file);

    let lines : Vec<String> = reader.lines()
                                .filter_map(|line| line.ok())
                                .collect();

    let re_range = Regex::new(r"^(.*): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
    let re_ticket = Regex::new(r"(\d+)").unwrap();

    let mut tests: Vec<FieldTest> = Vec::new();
    let mut my_ticket: Ticket = Vec::new();
    let mut nearby: Vec<Ticket> = Vec::new();
    let mut parse_phase = 0;

    for line in lines {
        if line == "" { continue; }
        if line == "your ticket:" {
            parse_phase = 1;
            continue;
        }
        if line == "nearby tickets:" {
            parse_phase = 2;
            continue;
        }

        if parse_phase == 0 && re_range.is_match(&line) {
            for cap in re_range.captures_iter(&line) {
                let r1b = cap[2].parse().unwrap();
                let r1e = cap[3].parse().unwrap();
                let r2b = cap[4].parse().unwrap();
                let r2e = cap[5].parse().unwrap();
                tests.push(FieldTest{
                    field: cap[1].to_string(),
                    test1: r1b..=r1e,
                    test2: r2b..=r2e,
                })
            }
        }

        if parse_phase == 1 && re_ticket.is_match(&line) {
            for cap in re_ticket.captures_iter(&line) {
                let nr: usize = cap[1].parse().unwrap();
                my_ticket.push(nr);
            }
        }

        if parse_phase == 2 && re_ticket.is_match(&line) {
            let mut n_ticket: Ticket = Vec::new();
            for cap in re_ticket.captures_iter(&line) {
                let nr: usize = cap[1].parse().unwrap();
                n_ticket.push(nr);
            }
            nearby.push(n_ticket);
        }
    }

    let result = solve_part_1(&tests, &nearby);
    println!("Result of part 1: {}", result);

    let result = solve_part_2(&tests, &nearby, my_ticket);
    println!("Result of part 2: {}", result);

    Ok(())
}

