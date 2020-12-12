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
                match nb_taken {
                    0 => next_gen[i][j] = Pos::Taken,
                    1 ..= 3 => {} ,
                    _ => next_gen[i][j] = Pos::Empty,
                }
            }
        }
        next_gen
    }

    fn get_nb_taken_neighbours(&self, r: usize, c: usize, nb_rows: usize, nb_cols: usize) -> usize {
        let mut nb_taken = 0;
        for i in -1..=1 {
            if r == 0 && i == -1 { continue; }
            if r == nb_rows - 1 && i == 1 { continue; }

            for j in -1..=1 {
                if i == 0 && j == 0 { continue; }
                if c == 0 && j == -1 { continue; }
                if c == nb_cols - 1 && j == 1 { continue; }
                
                let r_idx : usize = (r as i32 + i) as usize;
                let c_idx : usize = (c as i32 + j) as usize;
                if self[r_idx][c_idx] == Pos::Taken {
                    nb_taken += 1; 
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

fn solve_part_1(chairs: &Chairs) -> usize {
    let mut prev : Chairs = chairs.clone();
    loop {
        let next = prev.step();
        if prev == next {
            prev = next;
            break;
        }
        prev = next;
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

    let result = solve_part_1(&chairs);
    println!("Result of part 1: {}", result);

    /*
    let result = solve_part_2(&numbers,);
    println!("Result of part 2: {}", result);
    */

    Ok(())
}
