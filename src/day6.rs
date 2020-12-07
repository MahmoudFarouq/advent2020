use std::collections::{HashMap, HashSet};

use aoc_runner_derive::aoc;

#[aoc(day6, part1)]
fn day6_part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group
                .replace('\n', "")
                .chars()
                .collect::<HashSet<char>>()
                .len()
        })
        .sum()
}

#[aoc(day6, part2)]
fn day6_part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            let group_people = group.lines().count();
            group
                .replace('\n', "")
                .chars()
                .fold(HashMap::<char, usize>::new(), |mut acc, c| {
                    *acc.entry(c).or_insert(0) += 1;
                    acc
                })
                .iter()
                .filter(|(_, &count)| count == group_people)
                .count()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb";
        assert_eq!(day6_part1(input), 11);
    }

    #[test]
    fn test_part2() {
        let input = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb";
        assert_eq!(day6_part2(input), 6);
    }
}
