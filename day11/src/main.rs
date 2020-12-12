use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Pos {
    Floor,
    Empty,
    Taken,
}
type Row = Vec<Pos>;
type Chairs = Vec<Row>;

trait Board {
    fn step(&self) -> Self ;
    fn get_nb_taken_neighbours(&self, r: usize, c: usize, nb_rows: usize, nb_cols: usize) -> usize ;
}

impl Board for Chairs {
    fn step(&self) -> Self {
        let mut next_gen : Chairs = self.clone();
        let nb_cols = next_gen[0].len();
        let nb_rows = next_gen.len();

        for i in 0..nb_rows {
            for j in 0..nb_cols {
                if next_gen[i][j] == Pos::Floor { continue; }
                let nb_taken = self.get_nb_taken_neighbours(i, j, nb_rows, nb_cols);
                if i == 1 && j == 0 {
                    //dbg!(i, j, nb_taken);
                }
                match nb_taken {
                    0 => next_gen[i][j] = Pos::Taken,
                    1 ..= 4 => {} ,
                    _ => next_gen[i][j] = Pos::Empty,
                }
            }
        }
        //dbg!(&next_gen);
        next_gen
    }

    fn get_nb_taken_neighbours(&self, r: usize, c: usize, nb_rows: usize, nb_cols: usize) -> usize {
        let mut nb_taken = 0;

        // up
        if r > 0 {
            for i in (0..r).rev() {
                match self[i][c] {
                    Pos::Taken => { nb_taken += 1; break; },
                    Pos::Empty => { break; },
                    Pos::Floor => { continue; },
                }
            }
        }

        // up left
        if r > 0 && c > 0 {
            for (i, j) in (0..r).rev().zip((0..c).rev()) {
                match self[i][j] {
                    Pos::Taken => { nb_taken += 1; break; },
                    Pos::Empty => { break; },
                    Pos::Floor => { continue; },
                }
            }
        }

        // up right
        if r > 0 && c < nb_cols {
            for (i, j) in (0..r).rev().zip(c + 1..nb_cols) {
                match self[i][j] {
                    Pos::Taken => { nb_taken += 1; break; },
                    Pos::Empty => { break; },
                    Pos::Floor => { continue; },
                }
            }
        }

        // down
        if r < nb_rows {
            for i in (r+1..nb_rows) {
                match self[i][c] {
                    Pos::Taken => { nb_taken += 1; break; },
                    Pos::Empty => { break; },
                    Pos::Floor => { continue; },
                }
            }
        }

        // down left
        if r < nb_rows && c > 0 { 
            for (i, j) in (r+1..nb_rows).zip((0..c).rev()) {
                match self[i][j] {
                    Pos::Taken => { nb_taken += 1; break; },
                    Pos::Empty => { break; },
                    Pos::Floor => { continue; },
                }
            }
        }

        // down right
        if r < nb_rows && c < nb_cols {
            for (i, j) in (r+1..nb_rows).zip((c + 1..nb_cols)) {
                match self[i][j] {
                    Pos::Taken => { nb_taken += 1; break; },
                    Pos::Empty => { break; },
                    Pos::Floor => { continue; },
                }
            }
        }
        
        // left
        if c > 0 {
            for j in (0..c).rev() {
                match self[r][j] {
                    Pos::Taken => { nb_taken += 1; break; },
                    Pos::Empty => { break; },
                    Pos::Floor => { continue; },
                }
            }
        }

        // right
        if c < nb_cols {
            for j in (c + 1..nb_cols) {
                match self[r][j] {
                    Pos::Taken => { nb_taken += 1; break; },
                    Pos::Empty => { break; },
                    Pos::Floor => { continue; },
                }
            }
        }

        nb_taken
    }
}

trait SeatCount {
    fn count(&self, p: &Pos) -> usize ;
}

impl SeatCount for Row {
    fn count(&self, p: &Pos) -> usize {
        self.iter().filter(|c| *c == p).count()
    }
}

impl SeatCount for Chairs {
    fn count(&self, p: &Pos) -> usize {
        let mut total = 0;

        for row in self {
            total += row.count(p);
        }
        total
    }
}

fn line_to_seats(row : &str) -> Row {
    row.chars().map(|c| char_to_seat(&c)).collect()
}

fn char_to_seat(c : &char) -> Pos {
    match c {
        '.' => Pos::Floor,
        'L' => Pos::Empty,
        '#' => Pos::Taken,
        _   => Pos::Floor,
    }
}

fn solve_part_2(chairs: &Chairs) -> usize {
    let mut prev : Chairs = chairs.clone();
    let mut max_cnt = 0;
    loop {
        //if max_cnt == 2 { break; }
        let next = prev.step();
        if prev == next {
            prev = next;
            break;
        }
        prev = next;
        max_cnt += 1;
    }
    prev.count(&Pos::Taken)
}

fn main() -> io::Result<()> {
    let file = File::open("/home/rutger/old_home/arch-rutger/Programming/advent_of_code_2020/input/day11.txt")?;
    let reader = BufReader::new(file);

    let chairs : Chairs =  reader.lines()
                                .filter_map(|line| line.ok())
                                .map(|line| line_to_seats(&line))
                                .collect();

    let result = solve_part_2(&chairs);
    println!("Result of part 1: {}", result);

    Ok(())
}
