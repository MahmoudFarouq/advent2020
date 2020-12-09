use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Ordering;

#[aoc_generator(day9)]
fn parse_input_day9(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

#[aoc(day9, part1)]
fn day9_part1(input: &[usize]) -> Option<usize> {
    solve_part1(input, 25)
}

#[aoc(day9, part2)]
fn day9_part2(input: &[usize]) -> Option<usize> {
    solve_part2(input, 25)
}

fn solve_part1(input: &[usize], preamble: usize) -> Option<usize> {
    input
        .windows(preamble + 1)
        .find(|&window| search(&window[..window.len() - 1], window[window.len() - 1]).is_none())
        .unwrap()
        .last()
        .cloned()
}

fn solve_part2(input: &[usize], preamble: usize) -> Option<usize> {
    let invalid_number = solve_part1(input, preamble).unwrap();

    for pointer in 0..input.len() {
        let mut walking_sum = 0;
        for (last, number) in input.iter().enumerate().skip(pointer) {
            walking_sum += number;
            if walking_sum > invalid_number {
                break;
            }
            if walking_sum == invalid_number {
                return Some(
                    input[pointer..last + 1].iter().max().unwrap()
                        + input[pointer..last + 1].iter().min().unwrap(),
                );
            }
        }
    }

    None
}

fn search(input: &[usize], item: usize) -> Option<usize> {
    let mut copy = Vec::from(input);
    copy.sort_unstable();

    let mut first_index = 0;
    let mut second_index = copy.len() - 1;
    while first_index < second_index {
        let sum = copy[first_index] + copy[second_index];
        match sum.cmp(&item) {
            Ordering::Greater => second_index -= 1,
            Ordering::Less => first_index += 1,
            _ => return Some(0),
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        assert_eq!(solve_part1(&parse_input_day9(input), 5), Some(127));
    }

    #[test]
    fn test_part2() {
        let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        assert_eq!(solve_part2(&parse_input_day9(input), 5), Some(62));
    }
}
