use std::{convert::TryInto, str::FromStr};
use anyhow::{Context, Result, anyhow};

const INPUT: &'static str = include_str!("../input.txt");

const BOARD_SIZE: usize = 5;

#[derive(Clone, Debug)]
struct Entry {
    value: usize,
    marked: bool,
}

#[derive(Clone, Debug)]
struct Board {
    entries: [Entry; BOARD_SIZE * BOARD_SIZE],
}

impl FromStr for Board {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let entries = s.split_whitespace()
            .map(|num| num.parse::<usize>().map(|value| Entry { value, marked: false }))
            .collect::<Result<Vec<_>, _>>()?
            .try_into().map_err(|_| anyhow!("unexpected number of entries"))?;

        Ok(Board { entries })
    }
}

impl Board {
    fn mark(&mut self, number: usize) {
        for entry in &mut self.entries {
            if entry.value == number {
                entry.marked = true;
                // TODO: I think we can return early if
                // a number cannot be seen twice on the
                // same board.
            }
        }
    }

    fn is_winner(&self) -> bool {
        for row in 0..BOARD_SIZE {
            let mut all_marked = true;
            for col in 0..BOARD_SIZE {
                if !self.entries[(row * BOARD_SIZE) + col].marked {
                    all_marked = false;
                    break;
                }
            }
            if all_marked {
                return true;
            }
        }

        for col in 0..BOARD_SIZE {
            let mut all_marked = true;
            for row in 0..BOARD_SIZE {
                if !self.entries[(row * BOARD_SIZE) + col].marked {
                    all_marked = false;
                    break;
                }
            }
            if all_marked {
                return true;
            }
        }

        false
    }

    fn score(&self, number: usize) -> usize {
        self.entries.iter().filter(|e| !e.marked).map(|e| e.value).sum::<usize>() * number
    }
}

fn part1(numbers: &[usize], mut boards: Vec<Board>) -> Result<usize> {
    for number in numbers {
        for board in &mut boards {
            board.mark(*number);
            if board.is_winner() {
                return Ok(board.score(*number));
            }
        }
    }

    Err(anyhow!("Did not find a winner"))
}

fn part2(numbers: &[usize], mut boards: Vec<Board>) -> Result<usize> {
    let mut last_number = 0;
    let mut winners = vec![];
    for number in numbers {
        if boards.is_empty() {
            break
        }

        last_number = *number;

        for board in &mut boards {
            board.mark(*number);
        }
        
        let (new_winners, remaining): (Vec<Board>, Vec<Board>) = 
            boards.into_iter().partition(|b| b.is_winner());

        boards = remaining;
        winners.extend(new_winners.into_iter());
    }

    Ok(winners[winners.len()-1].score(last_number))
}

fn main() -> Result<()> {
    let mut parts = INPUT.split("\n\n");
    let numbers = parts.next().context("unexpected end of input")?
        .split(",")
        .map(|num| num.parse())
        .collect::<Result<Vec<_>, _>>()?;
    let boards: Vec<Board> = parts.map(|b| b.parse()).collect::<Result<_, _>>()?;

    println!("part 1: {}", part1(&numbers, boards.clone())?);
    println!("part 2: {}", part2(&numbers, boards)?);
    Ok(())
}
