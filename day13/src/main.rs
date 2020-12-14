use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use ring_algorithm::chinese_remainder_theorem;


fn solve_part_1(start: u64, busses: &Vec<u64>) -> u64 {
    let min: (u64, u64) = 
        busses.iter()
            .map(|bus| (start + bus - (start % bus), *bus))
            .min()
            .unwrap();

    let wait_time = min.0 - start;
    let bus = min.1;
    wait_time * bus
}

fn solve_part_2(m: &Vec<i64>, c: &Vec<i64>) -> i64 {
    dbg!(&m, &c);
    chinese_remainder_theorem::<i64>(&c, &m).unwrap()
}

fn main() -> io::Result<()> {
    let file = File::open("/home/rutger/old_home/arch-rutger/Programming/advent_of_code_2020/input/day13.txt")?;
    let reader = BufReader::new(file);

    let lines : Vec<String> = reader.lines()
                                .filter_map(|line| line.ok())
                                .collect();

    let start: u64 = lines[0].parse().unwrap();
    let busses: Vec<u64> = lines[1].split(",")
                                   .map(|x| x.parse())
                                   .filter_map(|n| n.ok())
                                   .collect();

    let result = solve_part_1(start, &busses);
    println!("Result of part 1: {}", result);

    let (results, moduli): (Vec<i64>, Vec<i64>) = 
        lines[1].split(",")
                .map(|x| -> std::result::Result<i64,_> { x.parse() }) // try to parse number or letter x
                .enumerate()                                          // add index in front ( which will be result mod number )
                .filter(|(_idx, parsed)| parsed.is_ok())              // filter those that are Ok()
                .map(|(idx, parsed)| (idx as i64, parsed.unwrap()))   // unwrap the number
                //.map(|(idx, modulus)| (modulus - idx, modulus))       // set correct congruence
                .unzip();                                             // store first part and second part of tuples in 2 Vecs
    let result = solve_part_2(&moduli, &results);
    println!("Result of part 2: {}", result);

    Ok(())
}

