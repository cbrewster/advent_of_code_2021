use std::collections::HashMap;

use anyhow::{Context, Result};

const INPUT: &str = include_str!("../input.txt");

fn part1(positions: &[i32]) -> Result<i32> {
    let min: i32 = *positions.iter().min().context("no positions")?;
    let max: i32 = *positions.iter().max().context("no positions")?;

    // O(n^2) sad :(
    // But it works for this problem :)
    let minimal_fuel = (min..=max).map(|alignment: i32| {
        positions.iter().map(|p| (p - alignment).abs()).sum::<i32>()
    }).min().expect("cannot get here");

    Ok(minimal_fuel)
}

// Use a cache to essentially memoize computing fuel cost.
// The solution for part 2 is still slow, but completes in a couple seconds.
fn compute_fuel(distance: i32, cache: &mut HashMap<i32, i32>) -> i32 {
    *cache.entry(distance).or_insert_with(|| {
        (1..=distance).sum::<i32>()
    })
}

fn part2(positions: &[i32]) -> Result<i32> {
    let min: i32 = *positions.iter().min().context("no positions")?;
    let max: i32 = *positions.iter().max().context("no positions")?;

    let mut cache = HashMap::new();

    // O(n^2) sad :(
    // But it works for this problem :)
    let minimal_fuel = (min..=max).map(|alignment: i32| {
        positions.iter().map(|p| {
            compute_fuel((p - alignment).abs(), &mut cache)
        }).sum::<i32>()
    }).min().expect("cannot get here");

    Ok(minimal_fuel)
}

fn main() -> Result<()> {
    let positions = INPUT.split(",").map(|n| n.parse()).collect::<Result<Vec<i32>, _>>()?;

    println!("part 1: {}", part1(&positions)?);
    println!("part 2: {}", part2(&positions)?);

    Ok(())
}
