use anyhow::{anyhow, Context, Result};
use std::{collections::HashSet, str::FromStr};

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Clone)]
struct Board {
    marks: HashSet<(usize, usize)>,
}

impl FromStr for Board {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let marks = s.lines()
            .map(|line| {
                let (x, y) = line.split_once(",").context("Expected ,")?;
                Ok((
                    x.parse()?,
                    y.parse()?,
                ))
            })
            .collect::<Result<HashSet<_>>>()?;
        Ok(Board { marks })
    }
}

#[derive(Debug, Clone)]
enum Fold {
    X(usize),
    Y(usize),
}

impl FromStr for Fold {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("fold along ").context("Expected 'fold along' prefix")?;
        let (axis, pos) = s.split_once("=").context("Expected '='")?;
        let pos: usize = pos.parse()?;

        match axis {
            "x" => Ok(Fold::X(pos)),
            "y" => Ok(Fold::Y(pos)),
            _ => Err(anyhow!("Expected 'x' or 'y'")),
        }
    }
}

fn do_fold(board: &mut Board, fold: &Fold) {
    let points_to_fold = board.marks.iter()
        .filter(|(x, y)| match fold {
            Fold::X(axis) => x > axis,
            Fold::Y(axis) => y > axis,
        })
        .cloned()
        .collect::<Vec<_>>();
    
    for point in points_to_fold {
        board.marks.remove(&point);
        match fold {
            Fold::X(axis) => {
                board.marks.insert((2 * axis - point.0, point.1));
            },
            Fold::Y(axis) => {
                board.marks.insert((point.0, 2 * axis - point.1));
            },
        }
    }
}

fn part1(mut board: Board, folds: &[Fold]) -> Result<usize> {
    let fold = folds.first().context("Expected at least one fold")?;
    do_fold(&mut board, fold);

    Ok(board.marks.len())
}

fn part2(mut board: Board, folds: &[Fold]) -> Result<()> {
    for fold in folds {
        do_fold(&mut board, fold);
    }
    // TODO: Debug print board.

    let x_max = *board.marks.iter().map(|(x, _)| x).max().context("Expected at least one point")?;
    let y_max = *board.marks.iter().map(|(x, _)| x).max().context("Expected at least one point")?;

    for y in 0..=y_max {
        for x in 0..=x_max {
            if board.marks.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("");
    }

    Ok(())
}

fn main() -> Result<()> {
    let (marks, folds) = INPUT.split_once("\n\n").context("Expected double newline")?;
    let board: Board = marks.parse()?;
    let folds = folds.lines().map(Fold::from_str).collect::<Result<Vec<_>>>()?;

    println!("part 1: {}", part1(board.clone(), &folds)?);
    println!("part 2:");
    part2(board, &folds)?;
    
    Ok(())
}
