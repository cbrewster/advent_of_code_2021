use anyhow::{Context, Result};
use std::{collections::{BTreeSet}, convert::TryFrom};

const INPUT: &'static str = include_str!("../input.txt");

type Display = BTreeSet<char>;

#[derive(Clone, Debug)]
struct Line {
    signals: Vec<Display>,
    output: Vec<Display>,
}

impl TryFrom<&str> for Line {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let (signals, output) =  s.split_once("|").context("expected |")?;
        let signals = signals.split_whitespace().map(|s| s.chars().collect()).collect();
        let output = output.split_whitespace().map(|s| s.chars().collect()).collect();

        Ok(Line { signals, output })
    }
}

impl Line {
    fn decode_output(&self) -> Result<usize> {
        let mut mapping: [Option<&Display>; 10] = [None; 10];

        // 1
        mapping[1] = Some(self.signals.iter().find(|d| d.len() == 2).context("unable to find 1")?);

        // 4
        mapping[4] = Some(self.signals.iter().find(|d| d.len() == 4).context("unable to find 4")?);

        // 7
        mapping[7] = Some(self.signals.iter().find(|d| d.len() == 3).context("unable to find 7")?);

        // 8
        mapping[8] = Some(self.signals.iter().find(|d| d.len() == 7).context("unable to find 8")?);

        // 3
        mapping[3] = Some(self.signals.iter().filter(|d| d.len() == 5).find(|d| d.intersection(mapping[1].unwrap()).count() == 2).context("unable to find 3")?);

        for d in self.signals.iter().filter(|d| d.len() == 5) {
            if (d.intersection(mapping[1].unwrap())).count() == 2 {
                mapping[3] = Some(d);
            } else if (d.intersection(mapping[4].unwrap())).count() == 2 {
                mapping[2] = Some(d);
            } else {
                mapping[5] = Some(d);
            }
        }

        for d in self.signals.iter().filter(|d| d.len() == 6) {
            if (d.intersection(mapping[1].unwrap())).count() == 1 {
                mapping[6] = Some(d);
            } else if (d.intersection(mapping[4].unwrap())).count() == 4 {
                mapping[9] = Some(d);
            } else {
                mapping[0] = Some(d);
            }
        }

        Ok(self.output.iter().fold(0, |acc, d| {
            let (num, _) = mapping.iter().enumerate().find(|(_, a)| **a == Some(d)).unwrap();
            (acc * 10) + num
        }))
    }
}

fn part1(lines: &[Line]) -> usize {
    // 1, 4, 7, 8 have unique count of segments enabled
    lines.iter().map(|l| l.output.iter().filter(|o| o.len() == 2 || o.len() == 4 || o.len() == 3 || o.len() == 7).count()).sum()
}

fn part2(lines: &[Line]) -> Result<usize> {
    let outputs = lines.iter().map(|l| l.decode_output()).collect::<Result<Vec<_>>>()?;
    Ok(outputs.iter().sum())
}
fn main() -> Result<()> {
    let lines = INPUT.lines().map(Line::try_from).collect::<Result<Vec<_>>>()?;

    println!("part 1: {}", part1(&lines));
    println!("part 2: {}", part2(&lines)?);
    Ok(())
}
