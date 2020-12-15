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
    let mut memory = Vec::from(numbers);
    let mut occurrences: HashMap<usize, Vec<usize>> = HashMap::new();

    let mut i = 0;
    for &num in memory.iter() {
        occurrences
            .entry(num)
            .or_default()
            .push(i);
        i += 1;
    }

    while i < nth {
        let occ = occurrences.get(memory.last().unwrap()).unwrap();

        let new;
        if occ.len() == 1 {
            new = 0;
        } else {
            new = occ[occ.len() - 1] - occ[occ.len() - 2];
        }

        memory.push(new);
        occurrences
            .entry(new)
            .or_default()
            .push(i);
        i += 1;
    }
    memory.last().cloned()
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
