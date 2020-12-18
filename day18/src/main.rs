use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug)]
enum Operand {
    Plus,
    Times,
    List(Vec<Operand>),
    Number(i64),
}

impl Operand {
    fn solve(&self) -> Option<i64> {
        match self {
            Operand::Plus => None,
            Operand::Times => None,
            Operand::List(l) => {
                let mut result: i64 = 1;
                let mut temp: i64 = l[0].solve().unwrap();
                for w in l[1..].chunks(2) {
                    let (op, right) = (&w[0], &w[1]);
                    match op {
                        Operand::Plus => temp += right.solve().unwrap(),
                        Operand::Times => {
                            result *= temp;
                            temp = right.solve().unwrap()
                        },
                        _ => unreachable!() ,
                    }
                }
                Some(result * temp)
            },
            Operand::Number(n) => Some(*n),
        }
    }
}

impl From<&str> for Operand {
    fn from(s: &str) -> Self {
        let mut v: Vec<Operand> = Vec::new();
        let mut cur_string: String = "".to_string();
        let mut open_brackets: usize = 0;
        for c in s.chars().into_iter() {
            match c {
                '0'..='9' => cur_string.push(c),
                '+' => {
                    if open_brackets == 0 {
                        v.push(Operand::Plus);
                    } else {
                        cur_string.push(c);
                    }
                },
                '*' => {
                    if open_brackets == 0 {
                        v.push(Operand::Times);
                    } else {
                        cur_string.push(c);
                    }
                },
                '(' => {
                    open_brackets += 1;
                    if open_brackets > 1 {
                        cur_string.push('(');
                    }
                },
                ')' => {
                    open_brackets -= 1;
                    if open_brackets == 0 {
                        v.push(cur_string.as_str().into());
                        cur_string = "".to_string();
                    } else {
                        cur_string.push(')');
                    }
                }
                ch => {
                    if open_brackets > 0 {
                        cur_string.push(ch)
                    } else if cur_string.len() > 0 && open_brackets == 0 {
                        v.push(Operand::Number(cur_string.parse().unwrap()));
                        cur_string = "".to_string();
                    }
                },
            }
        }
        if cur_string.len() > 0 {
            v.push(Operand::Number(cur_string.parse().unwrap()));
        }
        Operand::List(v)
    }
}

fn solve_part_1(lines: &Vec<String>) -> i64 {
    lines.iter()
         .map(|line| -> Operand { line.as_str().into() })
         .map(|op| op.solve().unwrap_or_default())
         .fold(0, |acc, n| acc + n)
}

fn main() -> io::Result<()> {
    let file = File::open("/home/rutger/old_home/arch-rutger/Programming/advent_of_code_2020/input/day18.txt")?;
    let reader = BufReader::new(file);

    let lines : Vec<String> = reader.lines()
                                .filter_map(|line| line.ok())
                                .collect();
    let result = solve_part_1(&lines);
    println!("Result of part 2 is {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let op: Operand = "1 + (2 * 3) + (4 * (5 + 6))".into();
        assert_eq!(op.solve(), Some(51));
    }

    #[test]
    fn test2() {
        let op: Operand = "2 * 3 + (4 * 5)".into();
        assert_eq!(op.solve(), Some(46));
    }

    #[test]
    fn test3() {
        let op: Operand = "5 + (8 * 3 + 9 + 3 * 4 * 3)".into();
        assert_eq!(op.solve(), Some(1_445));
    }

    #[test]
    fn test4() {
        let op: Operand = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".into();
        assert_eq!(op.solve(), Some(669_060));
    }

    #[test]
    fn test5() {
        let op: Operand = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".into();
        assert_eq!(op.solve(), Some(23_340));
    }
}
