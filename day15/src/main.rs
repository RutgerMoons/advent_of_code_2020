use std::collections::HashMap;

fn speak_nr(last_spoken: u64, spoken: &mut HashMap<u64, u64>, turn: u64) -> u64 {
    match spoken.insert(last_spoken, turn) {
        Some(old_turn) => turn - old_turn,
        None => 0, 
    }
}

fn solve_part_1(nrs: &Vec<u64>) -> u64 {
    let mut spoken = HashMap::new();
    let mut turn = 0;
    let mut last_spoken: u64 = 0;

    for nr in nrs {
        turn += 1;
        speak_nr(*nr, &mut spoken, turn);
        last_spoken = *nr;
    }

    while turn < 2020 {
        last_spoken = speak_nr(last_spoken, &mut spoken, turn);
        turn += 1;
    }

    last_spoken
}

fn main() {
    let input = "8,13,1,0,18,9";
    let nrs : Vec<u64> = input.split(",")
                                .map(|x| x.parse().unwrap())
                                .collect();

    let result = solve_part_1(&nrs);
    println!("Part 1: {}", result);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve_part_1(&vec![0,3,6]), 436);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve_part_1(&vec![1,3,2]), 1);
    }

    #[test]
    fn test_3() {
        assert_eq!(solve_part_1(&vec![2,1,3]), 10);
    }

    #[test]
    fn test_4() {
        assert_eq!(solve_part_1(&vec![2,3,1]), 78);
    }

    #[test]
    fn test_5() {
        assert_eq!(solve_part_1(&vec![3,2,1]), 438);
    }

    #[test]
    fn test_6() {
        assert_eq!(solve_part_1(&vec![1,2,3]), 27);
    }

    #[test]
    fn test_7() {
        assert_eq!(solve_part_1(&vec![3,1,2]), 1836);
    }
}