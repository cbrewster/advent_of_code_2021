use anyhow::{Context, Result};
use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

type InsertionRules = HashMap<(char, char), char>;

fn apply_rules(template: &[char], rules: &InsertionRules) -> Result<Vec<char>> {
    let new_elements = template.windows(2).map(|pair| {
        let key = (pair[0], pair[1]);
        rules.get(&key).context("expected insertion rule")
    }).collect::<Result<Vec<_>>>()?;

    let mut result = Vec::new();
    for (i, c) in template.iter().enumerate() {
        result.push(*c);
        if let Some(new_elem) = new_elements.get(i) {
            result.push(**new_elem);
        }
    }

    Ok(result)
}

fn part1(template: &[char], rules: &InsertionRules) -> Result<usize> {
    let mut template = template.to_owned();
    for _ in 0..10 {
        template = apply_rules(&template, &rules)?;
    }

    let buckets: HashMap<char, usize> = template.iter().fold(HashMap::new(), |mut acc, elem| {
        *acc.entry(*elem).or_default() += 1;
        acc
    });

    Ok(buckets.values().max().context("expected values")? - buckets.values().min().context("expected values")?)
}

fn part2(template: &[char], rules: &InsertionRules) -> Result<usize> {
    let mut pairs: HashMap<(char, char), usize> = template.windows(2).map(|w| (w[0], w[1]))
        .fold(HashMap::new(), |mut acc, key| {
            *acc.entry(key).or_default() += 1;
            acc
        });

    // The idea is to keep track of pairs instead of the whole string
    // This lets us bulk process each pair and provides an ubber bound on the amount
    // of work done per step, since there is only a finite number of element pair combinations.
    for _ in 0..40 {
        let mut new_pairs = HashMap::new();
        for (pair, count) in &pairs {
            let out = rules.get(pair).context("Expected rule")?;
            *new_pairs.entry((pair.0, *out)).or_default() += count;
            *new_pairs.entry((*out, pair.1)).or_default() += count;
        }
        pairs = new_pairs;
    }

    let buckets: HashMap<char, usize> = pairs.iter().fold(HashMap::new(), |mut acc, ((a, b), count)| {
        *acc.entry(*a).or_default() += count;
        *acc.entry(*b).or_default() += count;
        acc
    });
    
    // Divide by 2 because most are double counted
    Ok((buckets.values().max().context("expected values")? - buckets.values().min().context("expected values")?) / 2)
}

fn main() -> Result<()> {
    let (template, rules) = INPUT.split_once("\n\n").context("Expected 2 sections")?;
    let template = template.chars().collect::<Vec<_>>();
    let rules = rules.lines()
        .map(|line| {
            let (pair, out) = line.split_once(" -> ").context("Expected ->")?;
            let mut pair = pair.chars();
            let key = (pair.next().context("expected pair")?, pair.next().context("expected pair")?);
            let out = out.chars().next().context("expected out")?;
            Ok((key, out))
        })
        .collect::<Result<InsertionRules>>()?;
    
    println!("part 1: {}", part1(&template, &rules)?);
    println!("part 2: {}", part2(&template, &rules)?);
    Ok(())
}
