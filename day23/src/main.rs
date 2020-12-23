use std::collections::VecDeque;
use std::char;

fn solve_part_1(s: &str) -> String {
    let nb_iterations = 100;
    let mut cups: VecDeque<u32> = s.chars()
                                   .map(|c| c.to_digit(10).unwrap())
                                   .collect();
    let l = cups.len() as u32;
    let mut curr: u32 = cups[0];

    for _ in 0..nb_iterations {
        // pop until current is at the front
        while cups[0] != curr {
            let p = cups.pop_front().unwrap();
            cups.push_back(p);
        }

        // put current in the back
        let p = cups.pop_front().unwrap();
        cups.push_back(p);

        // take the next 3
        let (v1, v2, v3) = (cups.pop_front().unwrap(), cups.pop_front().unwrap(), cups.pop_front().unwrap());
        let next_cur = cups[0];

        // decide on value for the next 3
        let mut next_val = curr - 1;
        loop {
            if next_val == 0 {
                next_val += l;
            }
            
            if v1 == next_val || v2 == next_val || v3 == next_val {
                next_val -= 1;
            } else {
                break;
            }
        }

        // pop until next val
        while cups[0] != next_val {
            let p = cups.pop_front().unwrap();
            cups.push_back(p);
        }
        let p = cups.pop_front().unwrap();
        cups.push_back(p);

        cups.push_back(v1);
        cups.push_back(v2);
        cups.push_back(v3);
        
        // iteration done
        curr = next_cur;
    }
    
    while cups[cups.len() - 1] != 1 {
        let p = cups.pop_front().unwrap();
        cups.push_back(p);
    }

    let s: String = cups.into_iter()
        .take_while(|n| *n != 1)
        .map(|n| char::from_digit(n, 10).unwrap())
        .collect();
    s
}

fn main() {
    //let input = "389125467";
    let input = "538914762";

    let result = solve_part_1(&input);
    println!("Result of part 1: {}", result);
}
