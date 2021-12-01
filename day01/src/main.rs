use anyhow::Result;

const INPUT: &'static str = include_str!("../input.txt");

fn main() -> Result<()> {
    let depths = INPUT.lines()
        .map(|line| line.parse::<isize>())
        .collect::<Result<Vec<_>, _>>()?;
    
    println!("Part 1: {}", part1(&depths));
    println!("Part 2: {}", part2(&depths));
    
    Ok(())
}

fn part1(input: &[isize]) -> isize {
    input.windows(2).map(|w| if w[1] > w[0] { 1 } else { 0 }).sum()
}

fn part2(input: &[isize]) -> isize {
    let window_sums = input.windows(3).map(|w| w.iter().sum()).collect::<Vec<isize>>();
    window_sums.windows(2).map(|w| if w[1] > w[0] { 1 } else { 0 }).sum()
}
