use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;
use std::collections::HashMap;

enum Dir {
    E,
    W,
    NE,
    NW,
    SE,
    SW,
}

type Path = Vec<Dir>;
type Coord = (u64, u64);

fn solve_part_1(paths: &Vec<Path>) -> usize {
    let mut map: HashMap<Coord, u64> = HashMap::new(); 
    for path in paths {
        let mut x = 1_000_000;
        let mut y = 1_000_000;

        for d in path {
            match d {
                Dir::E => { x += 2; },
                Dir::W => { x -= 2; },
                Dir::NE => { x += 1; y += 1; },
                Dir::NW => { x -= 1; y += 1; },
                Dir::SE => { x += 1; y -= 1; },
                Dir::SW => { x -= 1; y -= 1; },
            }
        }

        *map.entry((x, y)).or_insert(0) += 1;
    }
    
    map.values().filter(|v| *v % 2 == 1).count()
}

fn main() -> io::Result<()> {
    let file = File::open("/home/rutger/old_home/arch-rutger/Programming/advent_of_code_2020/input/day24.txt")?;
    let reader = BufReader::new(file);

    let mut tile_directions : Vec<Path> = Vec::new();
    let re_dir = Regex::new(r"(e|w|ne|nw|se|sw)").unwrap();

    for l in reader.lines() {
        let line = l?;
        let mut path : Path = Vec::new();

        for cap in re_dir.captures_iter(&line) {
            let new_dir = match &cap[1] {
                "e" => Dir::E,
                "w" => Dir::W,
                "ne" => Dir::NE,
                "nw" => Dir::NW,
                "se" => Dir::SE,
                "sw" => Dir::SW,
                _ => unreachable!()
            };
            path.push(new_dir);
        }
        tile_directions.push(path);
    }

    let result = solve_part_1(&tile_directions);
    println!("Result of part 1 is {}", result);

    Ok(())
}
