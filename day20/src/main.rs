use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
struct Tile {
    tile_id: u64,
    up: u16,
    right: u16,
    down: u16,
    left: u16,
}

fn all_tiles (tile: Tile) -> Vec<Tile> {
    let curr_id = tile.tile_id;
    let up = tile.up;
    let right = tile.right;
    let down = tile.down;
    let left = tile.left;
    let mut tiles: Vec<Tile> = Vec::with_capacity(16);
    
    // regular rotated
    tiles.push(Tile {tile_id: curr_id, up: up,       right: right,   down: down,     left: left});
    tiles.push(Tile {tile_id: curr_id, up: left,     right: up,      down: right,    left: down});
    tiles.push(Tile {tile_id: curr_id, up: down,     right: left,    down: up,       left: right});
    tiles.push(Tile {tile_id: curr_id, up: right,    right: down,    down: left,     left: up});

    // mirror vertical rotated
    let mut vert_left = 0;
    let mut vert_right = 0;
    let mut left_copy = tile.left.clone();
    let mut right_copy = tile.right.clone();
    for _ in 0..10 {
        vert_left <<= 1;
        vert_right <<= 1;
        vert_left |= left_copy & 1;
        vert_right |= right_copy & 1;
        left_copy >>= 1;
        right_copy >>= 1;
    }
    
    tiles.push(Tile {tile_id: curr_id, up: down,       right: vert_right,   down: up,     left: vert_left});
    tiles.push(Tile {tile_id: curr_id, up: vert_left,     right: down,      down: vert_right,    left: up});
    tiles.push(Tile {tile_id: curr_id, up: up,     right: vert_left,    down: down,       left: vert_right});
    tiles.push(Tile {tile_id: curr_id, up: vert_right,    right: up,    down: vert_left,     left: down});

    // mirror horizontal rotated
    let mut hor_up = 0;
    let mut hor_down = 0;
    let mut up_copy = tile.up.clone();
    let mut down_copy = tile.down.clone();
    for _ in 0..10 {
        hor_down <<= 1;
        hor_up <<= 1;
        hor_down |= down_copy & 1;
        hor_up |= up_copy & 1;
        down_copy >>= 1;
        up_copy >>= 1;
    }

    tiles.push(Tile {tile_id: curr_id, up: hor_up,       right: left,   down: hor_down,     left: right});
    tiles.push(Tile {tile_id: curr_id, up: right,     right: hor_up,      down: left,    left: hor_down});
    tiles.push(Tile {tile_id: curr_id, up: hor_down,     right: right,    down: hor_up,       left: left});
    tiles.push(Tile {tile_id: curr_id, up: left,    right: hor_down,    down: right,     left: hor_up});

    // mirror vert + hor rotated
    tiles.push(Tile {tile_id: curr_id, up: hor_down,       right: vert_left,   down: hor_up,     left: vert_right});
    tiles.push(Tile {tile_id: curr_id, up: vert_right,     right: hor_down,      down: vert_left,    left: hor_up});
    tiles.push(Tile {tile_id: curr_id, up: hor_up,     right: vert_right,    down: hor_down,       left: vert_left});
    tiles.push(Tile {tile_id: curr_id, up: vert_left,    right: hor_up,    down: vert_right,     left: hor_down});

    tiles
}

// always check up and left border (unless top row)
fn solve_part_1(tiles: &Vec<Tile>) -> u64 {
    let dim: usize = ((tiles.len() / 16) as f64).sqrt() as usize; 
    let mut id_set: HashSet<u64> = HashSet::new();
    let mut placed_tiles: Vec<Tile> = Vec::with_capacity(dim * dim);
    for t in tiles {
        id_set = HashSet::with_capacity(dim * dim);
        id_set.insert(t.tile_id);
        placed_tiles = Vec::with_capacity(dim * dim);
        placed_tiles.push(t.clone());

        //dbg!(&t.tile_id);
        let x = solver_helper(&tiles, &mut id_set, &mut placed_tiles, &dim);
        if x > 0 {
            return x;
        }
    }    
    0
}

fn solver_helper(tiles: &Vec<Tile>, mut id_set: &mut HashSet<u64>, mut placed_tiles: &mut Vec<Tile>, dim: &usize) -> u64 {
    let l = placed_tiles.len();
    if l > 8 {
        println!("{}", l);
    }
    if l == (dim * dim) {
        //return corners
        return placed_tiles[0].tile_id *
               placed_tiles[dim - 1].tile_id *
               placed_tiles[dim * (dim - 1)].tile_id *
               placed_tiles[dim * dim - 1].tile_id;
    }

    for t in tiles {
        if id_set.contains(&t.tile_id) { continue; }

        // not top row. check top
        if l >= *dim && t.up != placed_tiles[l - dim].down { continue; }

        // not left col. check left
        if l % *dim != 1 && t.left != placed_tiles[l - 1].right { continue; }

        // test tile
        id_set.insert(t.tile_id);
        placed_tiles.push(t.clone());
        let x = solver_helper(&tiles, &mut id_set, &mut placed_tiles, &dim);
        if x > 0 { return x; }

        // clean up if not good
        id_set.remove(&t.tile_id);
        placed_tiles.pop();
    }
    0
}

fn main() -> io::Result<()> {
    //let file = File::open("/home/rutger/old_home/arch-rutger/Programming/advent_of_code_2020/input/day20_demo_2.txt")?;
    let file = File::open("/home/rutger/old_home/arch-rutger/Programming/advent_of_code_2020/input/day20_demo.txt")?;
    //let file = File::open("/home/rutger/old_home/arch-rutger/Programming/advent_of_code_2020/input/day20.txt")?;
    let reader = BufReader::new(file);

    let re_tile: Regex = Regex::new(r"^Tile (\d+):$").unwrap();
    let mut tiles: Vec<Tile> = Vec::new();
    let mut curr_id = 0;
    let mut tile_line_ctr = 0;
    let mut up = 0;
    let mut right = 0;
    let mut down = 0;
    let mut left = 0;

    for l in reader.lines() {
        let line = l?;
        if line.as_str() == "" {
            let regular = Tile {
                tile_id: curr_id,
                up: up,
                right: right,
                down: down,
                left: left,
            };

            // push tile
            tiles.extend(all_tiles(regular).into_iter());

            continue;
        }

        if re_tile.is_match(&line) {
            for cap in re_tile.captures_iter(&line) {
                curr_id = cap[1].parse().unwrap();
                tile_line_ctr = 0;
                up = 0;
                down = 0;
                left = 0;
                right = 0;
            }
            continue;
        }

        tile_line_ctr += 1;
        let chars: Vec<char> = line.chars().collect();
        if tile_line_ctr == 1 {
            for c in &chars {
                up <<= 1;
                if *c == '#' {
                    up |= 1;
                }
            }
        }

        if tile_line_ctr == 10 {
            for c in &chars {
                down <<= 1;
                if *c == '#' {
                    down |= 1;
                }
            }
        }

        left <<= 1;
        right <<= 1;
        if chars[0] == '#' {
            left |= 1;
        }

        if chars[9] == '#' {
            right |= 1;
        }
    }

    dbg!(&tiles);
    let result = solve_part_1(&tiles);
    println!("Solution to part 1: {}", result);

    Ok(())
}
