use std::str::FromStr;
use std::collections::HashMap;
use anyhow::{Context, Result};

const INPUT: &'static str = include_str!("../input.txt");

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(",").context("expected ,")?;
        Ok(Point { x: x.parse()?, y: y.parse()? })
    }
}

#[derive(Copy, Clone, Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once(" -> ").context("expected ->")?;
        Ok(Line { start: start.parse()?, end: end.parse()? })
    }
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }

    fn points(&self) -> Vec<Point> {
        let x1 = self.start.x.min(self.end.x);
        let x2 = self.start.x.max(self.end.x);

        let y1 = self.start.y.min(self.end.y);
        let y2 = self.start.y.max(self.end.y);

        if self.start.x == self.end.x {
            let x = self.start.x;
            (y1..=y2).map(move |y| Point { x, y }).collect()
        } else if self.start.y == self.end.y {
            let y = self.start.y;
            (x1..=x2).map(move |x| Point { x, y }).collect()
        } else {
            assert_eq!(x2 - x1, y2 - y1);
            let mut points = Vec::new();
            let mut point = Point { x: self.start.x, y: self.start.y };
            while point != self.end {
                points.push(point);
                if point.x < self.end.x {
                    point.x += 1;
                } else {
                    point.x -= 1;
                }

                if point.y < self.end.y {
                    point.y += 1;
                } else {
                    point.y -= 1;
                }
            }
            points.push(self.end);
            points
        }
    }
}

fn part1(lines: &[Line]) -> usize {
    let mut map: HashMap<Point, usize> = HashMap::new();
    for line in lines.iter().filter(|line| line.is_horizontal()) {
            // dbg!(&line, line.points().unwrap().collect::<Vec<_>>());
        for point in line.points() {
            *map.entry(point).or_default() += 1;
        }
    }

    map.values().filter(|count| **count > 1).count()
}

fn part2(lines: &[Line]) -> usize {
    let mut map: HashMap<Point, usize> = HashMap::new();
    for line in lines.iter() {
            // dbg!(&line, line.points().unwrap().collect::<Vec<_>>());
        for point in line.points() {
            *map.entry(point).or_default() += 1;
        }
    }

    map.values().filter(|count| **count > 1).count()
}

fn main() -> Result<()> {
    let lines: Vec<Line> = INPUT.lines().map(|l| l.parse()).collect::<Result<_>>()?;

    println!("part 1: {}", part1(&lines));
    println!("part 2: {}", part2(&lines));
    Ok(())
}
