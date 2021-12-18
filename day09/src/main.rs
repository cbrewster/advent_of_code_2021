use anyhow::{Context, Result};
use std::{collections::HashSet, str::FromStr};

const INPUT: &'static str = include_str!("../input.txt");

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

struct HeightMap {
    heights: Vec<Vec<u32>>,
}

impl FromStr for HeightMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let heights = s.lines().map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).context("not a number"))
                .collect::<Result<Vec<_>>>()
        }).collect::<Result<Vec<_>>>()?;

        Ok(HeightMap { heights })
    }
}

impl HeightMap {
    fn low_points(&self) -> Vec<(usize, usize)> {
        let mut points = vec![];
        for (row_i, row) in self.heights.iter().enumerate() {
            for (col_i, entry) in row.iter().enumerate() {
                let low_point = DIRECTIONS
                    .iter()
                    .filter_map(|d| apply_direction((col_i, row_i), *d))
                    .filter_map(|(col_j, row_j)| self.heights.get(row_j)?.get(col_j))
                    .all(|other| other > entry);
    
                if low_point {
                    points.push((col_i, row_i));
                }
            }
        }

        points
    }

    fn largest_basin(&self) -> usize {
        let low_points = self.low_points();
        let mut basins = Vec::new();

        for root in &low_points {
            let mut closed_list = HashSet::new();
            let mut open_list = vec![*root];

            while let Some(loc) = open_list.pop() {
                match self.heights.get(loc.1).and_then(|r| r.get(loc.0)) {
                    None | Some(9) => continue,
                    _ => {},
                };

                closed_list.insert(loc);
                open_list.extend(
                    DIRECTIONS.iter().filter_map(|d| apply_direction(loc, *d))
                        .filter(|d| !closed_list.contains(d))
                );
            }

            basins.push((closed_list.len(), closed_list));
        }

        basins.sort_by(|a, b| { b.0.cmp(&a.0) });

        basins[0..3].iter().fold(1, |acc, (size, _)| acc * size)
    }
}

fn apply_direction(pos: (usize, usize), dir: (i32, i32)) -> Option<(usize, usize)> {
    let col = pos.0 as i32 + dir.0;
    let row = pos.1 as i32 + dir.1;

    Some((
        col.try_into().ok()?, 
        row.try_into().ok()?, 
    ))
}

fn part1(height_map: &HeightMap) -> u32 {
    let low_points = height_map.low_points();
    low_points.iter().map(|(col, row)| height_map.heights[*row][*col] + 1).sum()
}

fn part2(height_map: &HeightMap) -> usize {
    height_map.largest_basin()
}

fn main() -> Result<()> {
    let height_map: HeightMap = INPUT.parse()?;

    println!("part 1: {}", part1(&height_map));
    println!("part 2: {}", part2(&height_map));
    Ok(())
}
