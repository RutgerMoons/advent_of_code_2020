
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
enum Rule {
    Literal(String),
    List(Vec<usize>),
    Or(Vec<usize>, Vec<usize>),
}
type RuleMap = HashMap<usize, Rule>;

fn parse_input(input: &Vec<String>) -> (RuleMap, Vec<&String>) {
    let mut parse_phase = 0;
    let mut cand: Vec<&String> = Vec::new();
    let mut rules_map: RuleMap = HashMap::new();
    let re_rule = Regex::new(r"^(\d+):(.*)$").unwrap();
    let re_literal = Regex::new("\"([a-z]+)\"").unwrap();
    let re_or = Regex::new(r"(.+) \| (.+)").unwrap();
    let re_list = Regex::new(r"(\d+)").unwrap();

    for rule in input.iter() {
        if parse_phase == 0 && rule.as_str() == "" {
            parse_phase += 1;
            continue;
        }

        if parse_phase > 0 {
            cand.push(rule);
        }

        for cap in re_rule.captures_iter(rule) {
            let begin: usize = cap[1].parse().unwrap();
            let rest = &cap[2];

            if re_literal.is_match(&rest) {
                for c in re_literal.captures_iter(&rest) {
                    let lit = c[1].to_string();
                    rules_map.insert(begin, Rule::Literal(lit));
                }

                continue;
            }

            if re_or.is_match(&rest) {
                for c in re_or.captures_iter(&rest) {
                    let left = &c[1];
                    let mut left_idx_list: Vec<usize> = Vec::new();
                    for c in re_list.captures_iter(&left) {
                        let rule: usize = c[1].parse().unwrap();
                        left_idx_list.push(rule);
                    }

                    let right = &c[2];
                    let mut right_idx_list: Vec<usize> = Vec::new();
                    for c in re_list.captures_iter(&right) {
                        let rule: usize = c[1].parse().unwrap();
                        right_idx_list.push(rule);
                    }
                    rules_map.insert(begin, Rule::Or(left_idx_list, right_idx_list));
                }

                continue;
            }

            let mut rules_idx_list: Vec<usize> = Vec::new();
            for c in re_list.captures_iter(&rest) {
                let rule: usize = c[1].parse().unwrap();
                rules_idx_list.push(rule);
            }
            if rules_idx_list.len() > 0 {
                rules_map.insert(begin, Rule::List(rules_idx_list));
            }
        }
    }

    (rules_map, cand)
}

fn produce_regex(rules: &RuleMap, start: usize, mut mappie: &mut HashMap<usize, String>) -> Regex {
    let mut rule = rules.get(&start).unwrap();
    let mut regex_str = "^".to_string();
    regex_str.push_str(&produce_regex_helper(&rules, &rule, 0, &mut mappie));
    regex_str.push('$');

    Regex::new(&regex_str).unwrap()
}

fn produce_regex_helper(rules: &RuleMap, start: &Rule, rule_idx: usize, mut mappie: &mut HashMap<usize, String>) -> String {
    let mut regex_str: String = "".to_string();
    match start {
        Rule::Literal(c) => {
            regex_str.push_str(c.as_str());
        },
        Rule::List(l) => {
            let mut matched = false;
            regex_str.push_str("(?:");
            for el in l {
                if *el == rule_idx {
                    regex_str.push_str(")+(?:");
                    continue;
                }
                let piece: String = produce_regex_helper(&rules, rules.get(&el).unwrap(), *el, &mut mappie);
                regex_str.push_str(&piece);
            }
            regex_str.push(')');
            if matched {
                regex_str.push('+');
            }
        },
        Rule::Or(left, right) => {
            regex_str.push_str("(?:");

            for el in left {
                let piece: String = produce_regex_helper(&rules, rules.get(&el).unwrap(), *el, &mut mappie);
                regex_str.push_str(&piece);
            }

            regex_str.push_str("|(?:");

            let mut matched = false;
            for el in right {
                if el == &rule_idx {
                    matched = true;
                    regex_str.push_str(")+(?:");
                    continue;
                }
                let piece: String = produce_regex_helper(&rules, rules.get(&el).unwrap(), *el, &mut mappie);
                regex_str.push_str(&piece);
            }

            if regex_str.chars().rev().next().unwrap() == ':' {
                regex_str.pop();
                regex_str.pop();
                regex_str.pop();
                matched = false;
            } else {
                regex_str.push(')');
            }
            if matched {
                regex_str.push('+');
            }

            regex_str.push_str(")");
        },
    }
    mappie.insert(rule_idx, regex_str.clone());

    regex_str
}

fn solve_part_1(input: &Vec<String>) -> usize {
    let (rules, candidates): (RuleMap, Vec<&String>) = parse_input(&input);
    let mut mappie: HashMap<usize, String> = HashMap::new();
    let regex: Regex = produce_regex(&rules, 0, &mut mappie);
    let mut test_regex: String = "^((?:".to_string();
    test_regex.push_str(&mappie.get(&42).unwrap().to_string());
    test_regex.push_str(")+)((?:");
    test_regex.push_str(&mappie.get(&31).unwrap().to_string());
    test_regex.push_str(")+)$");
    let str_42 = mappie.get(&42).unwrap();
    let re_42 = Regex::new(&str_42).unwrap();
    let str_31 = mappie.get(&31).unwrap();
    let re_31 = Regex::new(&str_31).unwrap();
    let re_test = Regex::new(&test_regex).unwrap();

    let mut total = 0;
    for cand in candidates.iter()
                          .filter(|c| re_test.is_match(&c))
                          .map(|c| c.to_string()) {
                           
        for cap in re_test.captures_iter(&cand) {
            let left = &cap[1].to_string();
            let right = &cap[2].to_string();
            let mut nb_42 = re_42.find_iter(&left).count();
            let mut nb_31 = re_31.find_iter(&right).count();
            if nb_42 > nb_31 && nb_31 > 0 {
                total += 1;
            }
            break;
        }
    }
    
    total
}
// CYK parser dan maar?

fn main() -> io::Result<()> {
    //let file = File::open("/home/rutger/old_home/arch-rutger/Programming/advent_of_code_2020/input/day19_demo_4.txt")?;
    //let file = File::open("/home/rutger/old_home/arch-rutger/Programming/advent_of_code_2020/input/day19_demo_3.txt")?;
    let file = File::open("/home/rutger/old_home/arch-rutger/Programming/advent_of_code_2020/input/day19.txt")?;
    let reader = BufReader::new(file);

    let lines : Vec<String> = reader.lines()
                                .filter_map(|line| line.ok())
                                .collect();
    let result = solve_part_1(&lines);
    println!("Result of part 2 is {}", result);
    Ok(())
}
