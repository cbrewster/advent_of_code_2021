use std::collections::HashMap;

use anyhow::Result;

const INPUT: &'static str = include_str!("../input.txt");

const BIRTH_PERIOD: usize = 7;

// Part 1: Naive, brute-force solution. Works for 80 days.
fn part1(fish: &[usize]) -> usize {
    let mut fish = fish.to_owned();

    for _ in 0..80 {
        let mut new_fish = Vec::new();
        for f in &mut fish {
            if *f > 0 {
                *f -= 1;
                continue;
            }

            *f = BIRTH_PERIOD - 1;
            new_fish.push((BIRTH_PERIOD - 1) + 2);
        }
        fish.extend(new_fish.into_iter());
    }

    fish.len()
}

// Part 2: Naive solution is much too slow and too large for 256 days.
// Instead bucket each fish into their days so we can bulk process them.
// We end up with a ~constant amount of work per day.
fn part2(fish: &[usize]) -> u64 {
    let mut groups: HashMap<usize, u64> = HashMap::new();

    for f in fish {
        *groups.entry(*f).or_default() += 1;
    }

    for _ in 0..256 {
        let mut new_groups = HashMap::new();
        for (f, count) in &groups {
            if *f > 0 {
                *new_groups.entry(*f - 1).or_default() += count;
                continue;
            }

            *new_groups.entry(BIRTH_PERIOD - 1).or_default() += count;
            *new_groups.entry((BIRTH_PERIOD - 1) + 2).or_default() += count;
        }

        groups = new_groups;
    }

    groups.values().sum()
}

fn main() -> Result<()> {
    let fish = INPUT.split(",").map(|f| f.parse()).collect::<Result<Vec<_>, _>>()?;

    println!("part 1: {}", part1(&fish));
    println!("part 2: {}", part2(&fish));

    Ok(())
}
