use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug, PartialEq, Clone)]
enum Cube {
    Active,
    Inactive,
}

impl Cube {
    fn is_active(&self) -> bool {
        match self {
            Cube::Active => true,
            _ => false
        }
    }
}

type CubeList = Vec<Vec<Vec<Vec<Cube>>>>;


fn pretty(list: &CubeList) {
    for i in 0..list.len() {
        println!("\nlayer {}", i);
        for j in 0..list[i].len() {
            for k in 0..list[i][j].len() {
                let s: String = list[i][j][k].iter().map( |c|
                    match c {
                        Cube::Active => '#',
                        Cube::Inactive => '.',
                    }
                ).collect();
                println!("{}", s);
            }
        }
    }
}

trait DeepClone {
    fn deepClone(&self) -> Self;
}

impl DeepClone for CubeList {
    fn deepClone(&self) -> Self {
        let mut cubes: CubeList = Vec::with_capacity(self.len());
        for i in 0..self.len() {
            let mut plane: Vec<Vec<Vec<Cube>>> = Vec::with_capacity(self[i].len());
            for j in 0..self[i].len() {
                let mut line: Vec<Vec<Cube>> = Vec::with_capacity(self[i][j].len());
                for k in 0..self[i][j].len() {
                    let mut subline: Vec<Cube> = Vec::with_capacity(self[i][j][k].len());
                    for l in 0..self[i][j][k].len() {
                        subline.push(self[i][j][k][l].clone());
                    }
                    line.push(subline);
                }
                plane.push(line);
            }
            cubes.push(plane);
        }
        cubes
    }
}

fn solve_part_1(cubes: &mut CubeList, n: usize) -> usize {
    let mut new_cubes: CubeList = cubes.deepClone();
    for _ in 0..n {
        // simulate here
        new_cubes = simulate_step(&new_cubes);
        //pretty(&new_cubes);
    }

    new_cubes.iter().fold(0, |acc, cube|
        acc + cube.iter().fold(0, |acc2, plane|
            acc2 + plane.iter().fold(0, |acc3, line|
                acc3 + line.iter().filter(|c| c.is_active()).count())))
}

fn simulate_step(cubes: &CubeList) -> CubeList {
    let mut new_cubes: CubeList = cubes.deepClone();
    for i in 1..cubes.len() - 1 {
        for j in 1..cubes[i].len() - 1 {
            for k in 1..cubes[i][j].len() - 1 {
                for l in 1..cubes[i][j][k].len() - 1 {
                    new_cubes[i][j][k][l] = Cube::Inactive;

                    let nb_active = count_active_neighbours(&cubes, i, j, k, l);
                    new_cubes[i][j][k][l] = match cubes[i][j][k][l] {
                        Cube::Active => {
                            if nb_active < 2 || nb_active > 3 {
                                Cube::Inactive
                            } else {
                                Cube::Active
                            }
                        },
                        Cube::Inactive => {
                            if nb_active == 3 {
                                Cube::Active
                            } else {
                                Cube::Inactive
                            }
                        },
                    }
                }
            }
        }
    }
    new_cubes
}

fn count_active_neighbours(cubes: &CubeList, x: usize, y: usize, z: usize, w: usize) -> usize {
    let mut count = 0;
    for i in 0..=2 {
        for j in 0..=2 {
            for k in 0..=2 {
                for l in 0..=2 {
                    if i == 1 && j == 1 && k == 1 && l == 1 {
                        continue;
                    }

                    if cubes[x + i - 1][y + j - 1][z + k - 1][w + l - 1]  == Cube::Active {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn main() -> io::Result<()> {
    let times = 6;
    let file = File::open("/home/rutger/old_home/arch-rutger/Programming/advent_of_code_2020/input/day17.txt")?;
    let reader = BufReader::new(file);

    let lines : Vec<String> = reader.lines()
                                .filter_map(|line| line.ok())
                                .collect();
    let dim = lines[0].len() + 2 * (times + 1);
    
    let mut hypercubes: CubeList = Vec::with_capacity(dim);
    
    for _ in 0..dim {
        let mut cubes: Vec<Vec<Vec<Cube>>> = Vec::with_capacity(dim);
        for _ in 0..dim {
            let mut p: Vec<Vec<Cube>> = Vec::with_capacity(dim);
            for _ in 0..dim {
                let z: Vec<Cube> = (0..dim).into_iter().map(|_| Cube::Inactive).collect();
                p.push(z);
            }
            cubes.push(p);
        }
        hypercubes.push(cubes);
    }

    for (idx, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                hypercubes[idx + times + 1][j + times + 1][times + 1][times + 1] = Cube::Active;
            }
        }
    }

    let result = solve_part_1(&mut hypercubes, times);
    println!("Result of part 1: {}", result);

    Ok(())
}
