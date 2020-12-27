use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

enum Dir {
    E,
    W,
    NE,
    NW,
    SE,
    SW,
}

type Path = Vec<Dir>;
type Coord = (i64, i64);

fn solve_part_1(paths: &Vec<Path>) -> HashMap<Coord, u64> {
    let mut map: HashMap<Coord, u64> = HashMap::new(); 
    for path in paths {
        let mut x = 0;
        let mut y = 0;

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
    map
}

fn solve_part_2(input_coords: &HashMap<Coord, u64>) -> usize {
    let mut coords: HashSet<Coord> = input_coords.iter()
                                             .filter(|(_, val)| *val % 2 == 1)  // black pairs
                                             .map(|((x, y), _)| (*x, *y))       // only the coord
                                             .collect();
    for i in 0..100 {
        let mut new_map: HashMap<Coord, u64> = HashMap::new();
        for coord in coords.iter() {
            // dereference
            let (x, y) = *coord;

            // neighbors + 1
            *new_map.entry((x + 2, y)).or_insert(0) += 1;
            *new_map.entry((x - 2, y)).or_insert(0) += 1;
            *new_map.entry((x + 1, y + 1)).or_insert(0) += 1;
            *new_map.entry((x + 1, y - 1)).or_insert(0) += 1;
            *new_map.entry((x - 1, y + 1)).or_insert(0) += 1;
            *new_map.entry((x - 1, y - 1)).or_insert(0) += 1;
        }
        let mut new_coords = HashSet::new();

        for (coord, ctr) in new_map {
            // black
            if coords.contains(&coord) {
                // stays black
                if ctr > 0 && ctr < 3 {
                    new_coords.insert(coord.clone());
                }
            }
            else {
                // white flips to black
                if ctr == 2 {
                    new_coords.insert(coord.clone());
                }
            }
        }
        coords = new_coords;
    }
    coords.len()
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

    let map = solve_part_1(&tile_directions);
    let result = map.values().filter(|v| *v % 2 == 1).count();
    println!("Result of part 1 is {}", result);

    let result = solve_part_2(&map);
    println!("Result of part 2 is {}", result);
    Ok(())
}
