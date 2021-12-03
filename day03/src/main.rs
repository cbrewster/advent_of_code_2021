use anyhow::{Context, Result};

const INPUT: &'static str = include_str!("../input.txt");

fn main() -> Result<()> {
    let numbers = INPUT.lines()
        .map(|l| isize::from_str_radix(l, 2))
        .collect::<Result<Vec<_>, _>>()?;
    let width = INPUT.lines().next().context("expected data")?.len();

    println!("part 1: {}", part1(&numbers, width));
    println!("part 2: {}", part2(&numbers, width));

    Ok(())
}

// every solution is just a handful of folds away...
fn part1(numbers: &[isize], width: usize) -> isize {
    let buckets = numbers.iter().fold(vec![0; width], |mut buckets, number| {
        for i in 0..width {
            if (1 << i & number) != 0 {
                buckets[i] += 1;
            }
        }
        buckets
    });

    let gamma = buckets.iter().enumerate().fold(0, |gamma, (i, count)| {
        if *count > (numbers.len() / 2) {
            gamma | 1 << i
        } else { 
            gamma 
        }
    });

    let epsilon = buckets.iter().enumerate().fold(0, |epsilon, (i, count)| {
        if *count < (numbers.len() / 2) {
            epsilon | 1 << i
        } else {
            epsilon
        }
    });

    gamma * epsilon
}

fn one_is_most_common_bit(numbers: &[isize], pos: usize) -> bool {
    let ones = numbers.iter().fold(0, |count, number| {
        if (1 << pos & number) != 0 {
            count + 1
        } else {
            count
        }
    });
    ones as f64 >= (numbers.len() as f64 / 2.0)
}

fn zero_is_least_common_bit(numbers: &[isize], pos: usize) -> bool {
    let zeros = numbers.iter().fold(0, |count, number| {
        if (1 << pos & number) == 0 {
            count + 1
        } else {
            count
        }
    });
    zeros as f64 <= (numbers.len() as f64 / 2.0)
}

fn part2(numbers: &[isize], width: usize) -> isize {
    let mut oxygen_open_list = numbers.to_vec();
    // Whyyy is the first bit really the highest bit??
    let mut i = (width as isize) - 1;
    while oxygen_open_list.len() > 1 && i >= 0 {
        let is_one = one_is_most_common_bit(&oxygen_open_list, i as usize);
        oxygen_open_list.retain(|number| {
            if is_one {
                // 1 is most common, keep where it has a 1
                1 << i & number != 0
            } else {
                // 0 is most common, keep where it has a 0
                1 << i & number == 0
            }
        });
        i -= 1;
    }

    let mut co2_open_list = numbers.to_vec();
    let mut i = (width as isize) - 1;
    while co2_open_list.len() > 1 && i >= 0 {
        let is_zero = zero_is_least_common_bit(&co2_open_list, i as usize);
        co2_open_list.retain(|number| {
            if is_zero {
                // 0 is least common, keep where it has a 0
                1 << i & number == 0
            } else {
                // 1 is least common, keep where it has a 1
                1 << i & number != 0
            }
        });
        i -= 1;
    }

    // TODO: Will panic if no result is found
    oxygen_open_list[0] * co2_open_list[0]
}