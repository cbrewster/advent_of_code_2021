use anyhow::{Context, Result};
use std::fmt::Debug;

const INPUT: &'static str = include_str!("../input.txt");

const DIRECTIONS: &[(isize, isize)] = &[
    (-1,  1), (0,  1), (1,  1),
    (-1,  0),          (1,  0),
    (-1, -1), (0, -1), (1, -1),
];

#[derive(Clone)]
struct Grid([[u32; 10]; 10]);

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Grid:")?;
        for row in &self.0 {
            for entry in row {
                if *entry == 0 {
                    write!(f, "\x1b[92m{}\x1b[0m", entry)?;
                } else {
                    write!(f, "{}", entry)?;
                }
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

fn apply_direction(pos: (usize, usize), dir: (isize, isize)) -> Option<(usize, usize)> {
    let col = pos.0 as isize + dir.0;
    let row = pos.1 as isize + dir.1;

    Some((
        col.try_into().ok()?, 
        row.try_into().ok()?, 
    ))
}

fn part1(mut grid: Grid) -> usize {
    let mut flash_count = 0;

    for _ in 0..100 {
        for row in &mut grid.0 {
            for entry in row {
                *entry += 1;
            }
        }

        // Note: At this point, no octopuses have an energy level of 0.
        // We will use 0 to mark the octopus has flashed once this iteration.

        let mut flash_list = Vec::new();
        for y in 0..10 {
            for x in 0..10 {
                if grid.0[y][x] > 9 {
                    flash_list.push((x, y));
                }
            }
        }

        while let Some(pos) = flash_list.pop() {
            // Already flashed
            if grid.0[pos.1][pos.0] == 0 {
                continue;
            }

            flash_count += 1;

            grid.0[pos.1][pos.0] = 0;
            let neighbors = DIRECTIONS.iter().filter_map(|dir| apply_direction(pos, *dir)).collect::<Vec<_>>();

            for (x, y) in neighbors {
                // Already flashed
                match grid.0.get(y).and_then(|row| row.get(x)) {
                    None | Some(0) => continue,
                    _ => {},
                };

                grid.0[y][x] += 1;
                if grid.0[y][x] > 9 {
                    flash_list.push((x, y));
                }
            }
        }
    }

    flash_count
}

fn part2(mut grid: Grid) -> usize {
    for i in 1.. {
        for row in &mut grid.0 {
            for entry in row {
                *entry += 1;
            }
        }

        // Note: At this point, no octopuses have an energy level of 0.
        // We will use 0 to mark the octopus has flashed once this iteration.

        let mut flash_list = Vec::new();
        for y in 0..10 {
            for x in 0..10 {
                if grid.0[y][x] > 9 {
                    flash_list.push((x, y));
                }
            }
        }

        while let Some(pos) = flash_list.pop() {
            // Already flashed
            if grid.0[pos.1][pos.0] == 0 {
                continue;
            }

            grid.0[pos.1][pos.0] = 0;
            let neighbors = DIRECTIONS.iter().filter_map(|dir| apply_direction(pos, *dir)).collect::<Vec<_>>();

            for (x, y) in neighbors {
                // Already flashed
                match grid.0.get(y).and_then(|row| row.get(x)) {
                    None | Some(0) => continue,
                    _ => {},
                };

                grid.0[y][x] += 1;
                if grid.0[y][x] > 9 {
                    flash_list.push((x, y));
                }
            }
        }

        if grid.0.iter().flat_map(|r| r.iter()).all(|e| *e == 0) {
            return i;
        }
    }

    0
}

fn main() -> Result<()> {
    // okay, maybe iterators are a bit too messy for this directly...
    let grid = Grid(INPUT.lines()
        .map(|l| {
            Ok(l.
                chars()
                .map(|c| Ok(c.to_digit(10).context("invalid number")?))
                .collect::<Result<Vec<_>>>()?
                .try_into().ok().context("unexpected col count")?)
        })
        .collect::<Result<Vec<[u32; 10]>>>()?
        .try_into().ok().context("unexpected row count")?);
    
    println!("part 1: {}", part1(grid.clone()));
    println!("part 2: {}", part2(grid.clone()));
    Ok(())
}
