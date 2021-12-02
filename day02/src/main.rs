use anyhow::{anyhow, Context, Result};
use std::str::FromStr;

const INPUT: &'static str = include_str!("../input.txt");

type Command = (Direction, isize);

#[derive(Debug, Clone, Copy)]
enum Direction {
    Forward,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "forward" => Ok(Direction::Forward),
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            _ => Err(anyhow!("Invalid direction")),
        }
    }
}

fn main() -> Result<()> {
    let commands = INPUT.lines()
        .map(|l| {
            let (dir, amount) = l.split_once(" ").context("unexpected format")?;
            let dir = dir.parse()?;
            let amount = amount.parse()?;
            Ok((dir, amount))
        })
        .collect::<Result<Vec<Command>>>()?;

    println!("part 1: {}", part1(&commands));
    println!("part 2: {}", part2(&commands));
    
    Ok(())
}

fn part1(input: &[Command]) -> isize {
    let (horizontal, depth) = input.iter().fold((0, 0), |(h, d), (dir, amount)| {
        match dir {
            Direction::Forward => (h + amount, d),
            Direction::Up => (h, d - amount),
            Direction::Down => (h, d + amount),
        }
    });
    horizontal * depth
}

fn part2(input: &[Command]) -> isize {
    let (horizontal, depth, _) = input.iter().fold((0, 0, 0), |(h, d, aim), (dir, amount)| {
        match dir {
            Direction::Forward => (h + amount, d + (amount * aim), aim),
            Direction::Up => (h, d, aim - amount),
            Direction::Down => (h, d, aim + amount),
        }
    });
    horizontal * depth
}
