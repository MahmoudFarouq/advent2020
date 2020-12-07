use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

use regex::Regex;

// A Bag can contain 'count' bags of color 'color'
#[derive(Debug)]
struct Node {
    count: usize,
    color: String,
}

impl Node {
    fn new(count: usize, color: String) -> Self {
        Self { count, color }
    }
}

#[aoc_generator(day7)]
fn parse_input_day7(input: &str) -> HashMap<String, Vec<Node>> {
    let all_regex = Regex::new(r"([\s\w]+?) bags contain ([\s\w,]+?)\.\n?").unwrap();
    let sentence_regex = Regex::new(r"(\d+) ([\s\w]+?) bag").unwrap();

    let mut graph = HashMap::<String, Vec<Node>>::new();
    all_regex
        .captures_iter(input)
        .map(|capture| {
            capture[2]
                .split(", ")
                .map(|sentence| {
                    sentence_regex
                        .captures_iter(sentence)
                        .map(|edge| {
                            graph
                                .entry(capture[1].to_string())
                                .or_default()
                                .push(Node::new(edge[1].parse().unwrap(), edge[2].to_string()));
                        })
                        .for_each(drop);
                })
                .for_each(drop);
        })
        .for_each(drop);
    graph
}

fn reverse_graph(graph: &HashMap<String, Vec<Node>>) -> HashMap<String, Vec<Node>> {
    let mut new_graph = HashMap::<String, Vec<Node>>::new();
    graph
        .iter()
        .map(|(key, nodes)| {
            nodes
                .iter()
                .map(|node| {
                    new_graph
                        .entry(node.color.clone())
                        .or_default()
                        .push(Node::new(node.count, key.to_string()))
                })
                .for_each(drop);
        })
        .for_each(drop);
    new_graph
}

#[aoc(day7, part1)]
fn day7_part1(graph: &HashMap<String, Vec<Node>>) -> Option<usize> {
    let graph = reverse_graph(graph);

    let search_for = "shiny gold";
    let mut queue = Vec::new();
    let mut result = HashSet::new();

    queue.push(search_for);

    while !queue.is_empty() {
        if let Some(value) = graph.get(queue.pop().unwrap()) {
            value
                .iter()
                .map(|node| {
                    queue.push(&node.color);
                    result.insert(&node.color);
                })
                .for_each(drop);
        }
    }

    Some(result.len())
}

#[aoc(day7, part2)]
fn day7_part2(graph: &HashMap<String, Vec<Node>>) -> Option<usize> {
    let search_for = "shiny gold";
    Some(solve(graph, search_for))
}

fn solve(graph: &HashMap<String, Vec<Node>>, search_for: &str) -> usize {
    graph
        .get(search_for)
        .map(|nodes| {
            nodes
                .iter()
                .map(|node| node.count + node.count * solve(graph, &node.color))
                .sum()
        })
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        assert_eq!(day7_part1(&parse_input_day7(input)), Some(4));
    }

    #[test]
    fn test_part2() {
        let input = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
        assert_eq!(day7_part2(&parse_input_day7(input)), Some(126));
    }
}
