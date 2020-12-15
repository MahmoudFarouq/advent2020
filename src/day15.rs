use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day15)]
fn parse_input_day13(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|number| number.parse().unwrap())
        .collect()
}

#[aoc(day15, part1)]
fn day15_part1(numbers: &[usize]) -> Option<usize> {
    solver(numbers, 2020)
}

#[aoc(day15, part2)]
fn day15_part2(numbers: &[usize]) -> Option<usize> {
    solver(numbers, 30000000)
}

fn solver(numbers: &[usize], nth: usize) -> Option<usize> {
    let mut occurrences: HashMap<usize, Vec<usize>> = HashMap::new();

    numbers
        .iter()
        .enumerate()
        .for_each(|(i, &num)| occurrences.entry(num).or_default().push(i));

    (numbers.len()..nth)
        .into_iter()
        .fold(numbers.last().unwrap().clone(), |latest, i| {
            let occ = occurrences.get(&latest).unwrap();
            let latest = if occ.len() == 1 {
                0
            } else {
                occ[occ.len() - 1] - occ[occ.len() - 2]
            };
            occurrences.entry(latest).or_default().push(i);
            latest
        })
        .clone()
        .into()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "0,3,6";
        assert_eq!(day15_part1(&parse_input_day13(input)), Some(436));

        let input = "1,3,2";
        assert_eq!(day15_part1(&parse_input_day13(input)), Some(1));

        let input = "2,1,3";
        assert_eq!(day15_part1(&parse_input_day13(input)), Some(10));

        let input = "1,2,3";
        assert_eq!(day15_part1(&parse_input_day13(input)), Some(27));

        let input = "2,3,1";
        assert_eq!(day15_part1(&parse_input_day13(input)), Some(78));

        let input = "3,2,1";
        assert_eq!(day15_part1(&parse_input_day13(input)), Some(438));

        let input = "3,1,2";
        assert_eq!(day15_part1(&parse_input_day13(input)), Some(1836));
    }
}
