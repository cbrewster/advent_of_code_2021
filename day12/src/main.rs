use anyhow::{Context, Result};
use std::{collections::{HashMap, HashSet}, convert::TryFrom, fmt};

const INPUT: &str = include_str!("../input.txt");

const START: &str = "start";
const END: &str = "end";

type Node<'a> = &'a str;

#[derive(Clone)]
struct Path<'a>(Vec<Node<'a>>);

impl<'a> fmt::Debug for Path<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, node) in self.0.iter().enumerate() {
            write!(f, "{}", node)?;
            if i != self.0.len() - 1 {
                write!(f, "->")?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Graph<'a> {
    edges: HashMap<Node<'a>, HashSet<Node<'a>>>,
}

impl<'a> TryFrom<&'a str> for Graph<'a> {
    type Error = anyhow::Error;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let edges: Vec<(Node, Node)> = s.lines().map(|line| line.split_once("-").context("expected -")).collect::<Result<_>>()?;

        let edges = edges.iter().fold(HashMap::<Node, HashSet<Node>>::new(), |mut map, (from, to)| {
            if *to != START {
                map.entry(*from).or_default().insert(to);
            }
            if *from != START {
                map.entry(*to).or_default().insert(from);
            }
            map
        });

        Ok(Graph { edges })
    }
}

fn find_paths_1<'a>(graph: &Graph<'a>, path: Path<'a>) -> Vec<Path<'a>> {
    let current = path.0.last().expect("path should never be empty");
    if *current == END {
        return vec![path];
    }

    let options = match graph.edges.get(current) {
        Some(options) => options,
        None => return Vec::new(),
    };

    options.iter()
        .filter(|next| {
            next.to_uppercase() == **next || !path.0.contains(next)
        })
        .flat_map(|next| {
            let mut new_path = path.clone();
            new_path.0.push(next);
            find_paths_1(graph, new_path)
        })
        .collect()
}

fn part1(graph: &Graph) -> usize {
    find_paths_1(graph, Path(vec![START])).len()
}

fn find_paths_2<'a>(graph: &Graph<'a>, path: Path<'a>, small_twice: bool) -> Vec<Path<'a>> {
    let current = path.0.last().expect("path should never be empty");
    if *current == END {
        return vec![path];
    }

    let options = match graph.edges.get(current) {
        Some(options) => options,
        None => return Vec::new(),
    };

    options.iter()
        .filter(|next| {
            !small_twice || next.to_uppercase() == **next || !path.0.contains(next)
        })
        .flat_map(|next| {
            let mut new_path = path.clone();
            new_path.0.push(next);
            let small_twice = small_twice || (next.to_lowercase() == **next && path.0.contains(next));
            find_paths_2(graph, new_path, small_twice)
        })
        .collect()
}

fn part2(graph: &Graph) -> usize {
    find_paths_2(graph, Path(vec![START]), false).len()
}

fn main() -> Result<()> {
    let graph: Graph = INPUT.try_into()?;

    println!("part 1: {}", part1(&graph));
    println!("part 2: {}", part2(&graph));

    Ok(())
}
