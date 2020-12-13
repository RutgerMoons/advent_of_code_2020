use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Instr {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn rotate_right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Instruction {
    instr : Instr,
    amount : i32,
}

impl From<String> for Instruction {
    fn from(s: String) -> Self {
        let instr: Instr = match &s[0..1] {
            "N" => Instr::North,
            "E" => Instr::East,
            "S" => Instr::South,
            "W" => Instr::West,
            "L" => Instr::Left,
            "R" => Instr::Right,
            "F" => Instr::Forward,
            _ => unreachable!(),
        };
        let amount : i32 = s[1..].parse().unwrap();
        Instruction { instr : instr, amount : amount }
    }
}

impl From<Direction> for Instr {
    fn from(d: Direction) -> Self {
        match d {
            Direction::North => Instr::North,
            Direction::East => Instr::East,
            Direction::South => Instr::South,
            Direction::West => Instr::West,
        }
    }
}

struct Ship {
    x : i32,
    y : i32,
    dir : Direction,
    wayx: i32,
    wayy: i32,
}

impl Ship {
    fn move_ship(&mut self, instruction : &Instruction) {
        match instruction.instr {
            Instr::North => self.wayy += instruction.amount,
            Instr::South => self.wayy -= instruction.amount,
            Instr::East => self.wayx += instruction.amount,
            Instr::West => self.wayx -= instruction.amount,
            Instr::Left => self.rotate_left(instruction.amount),
            Instr::Right => self.rotate_right(instruction.amount),
            Instr::Forward => {
                self.x += instruction.amount * self.wayx;
                self.y += instruction.amount * self.wayy;
            },
        }
    }

    fn rotate_left(&mut self, degrees: i32) {
        self.rotate_right(360 - degrees);
    }

    fn rotate_right(&mut self, degrees: i32) {
        let times = degrees / 90;
        let mut new_dir = self.dir;
        let mut new_wayx = self.wayx;
        let mut new_wayy = self.wayy;
        for _ in 0..times {
            new_dir = new_dir.rotate_right();
            let wayx_copy = new_wayx;
            new_wayx = new_wayy;
            new_wayy = - wayx_copy;
        }
        self.dir = new_dir;
        self.wayx = new_wayx;
        self.wayy = new_wayy;
    }
}

impl Default for Ship {
    fn default() -> Self {
        Ship {
            x: 0,
            y: 0,
            dir: Direction::East,
            wayx: 10,
            wayy: 1,
        }
    }
}

fn solve_part_2(route : &Vec<Instruction>) -> i32 {
    let mut ship = Ship::default();
    for instruction in route.iter() {
        ship.move_ship(instruction);
    }
    ship.x.abs() + ship.y.abs()
}

fn main() -> io::Result<()> {
    let file = File::open("/home/rutger/old_home/arch-rutger/Programming/advent_of_code_2020/input/day12.txt")?;
    let reader = BufReader::new(file);

    let route : Vec<Instruction> = reader.lines()
                                .filter_map(|line| line.ok())
                                .map(|line| line.into())
                                .collect();

    let result = solve_part_2(&route);
    println!("Result of part 2: {}", result);

    Ok(())
}
