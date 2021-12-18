use anyhow::{anyhow, Context, Result};

const INPUT: &'static str = include_str!("../input.txt");

fn first_error(line: &str) -> Result<Option<char>> {
    let mut stack = Vec::new();

    for c in line.chars() {
        match c {
            '(' | '<' | '{' | '[' => stack.push(c),
            ')' | '>' | '}' | ']' => {
                let opener = stack.pop().context("unexpected closer")?;
                match (opener, c) {
                    ('(', ')') | ('<', '>') | ('{', '}') | ('[', ']') => {},
                    _ => return Ok(Some(c)),
                }
            },
            _ => return Err(anyhow!("unexpected input")),
        }
    }

    Ok(None)
}

fn score_line(line: &str) -> Result<Option<usize>> {
    let mut stack = Vec::new();

    for c in line.chars() {
        match c {
            '(' | '<' | '{' | '[' => stack.push(c),
            ')' | '>' | '}' | ']' => {
                let opener = stack.pop().context("unexpected closer")?;
                match (opener, c) {
                    ('(', ')') | ('<', '>') | ('{', '}') | ('[', ']') => {},
                    _ => return Ok(None),
                }
            },
            _ => return Err(anyhow!("unexpected input")),
        }
    }

    let mut score = 0;
    for c in stack.iter().rev() {
        score *= 5;
        match c {
            '(' => score += 1,
            '[' => score += 2,
            '{' => score += 3,
            '<' => score += 4,
            _ => return Err(anyhow!("unexpected input")),
        }
    }

    Ok(Some(score))
}

fn part1(lines: &[&'static str]) -> Result<usize> {
    let mut score = 0;
    for line in lines {
        match first_error(line)? {
            Some(')') => score += 3,
            Some(']') => score += 57,
            Some('}') => score += 1197,
            Some('>') => score += 25137,
            _ => {},
        }
    }

    Ok(score)
}

fn part2(lines: &[&'static str]) -> Result<usize> {
    let scores = lines.iter().map(|l| score_line(l)).collect::<Result<Vec<_>>>()?;
    let mut scores = scores.iter().filter_map(|s| *s).collect::<Vec<_>>();
    scores.sort();

    Ok(scores[scores.len() / 2])
}

fn main() -> Result<()> {
    let lines = INPUT.lines().collect::<Vec<_>>();

    println!("part 1: {}", part1(&lines)?);
    println!("part 2: {}", part2(&lines)?);
    Ok(())
}
