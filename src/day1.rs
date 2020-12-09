use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Ordering;
use std::num::ParseIntError;

#[aoc_generator(day1)]
fn parse_input_day1(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.lines().map(|l| l.parse()).collect()
}

#[aoc(day1, part1)]
fn day1_part1(input: &[i32]) -> Option<i32> {
    let mut numbers = vec![0; input.len()];
    numbers.clone_from_slice(input);
    numbers.sort_unstable();

    let needed_sum = 2020;
    let mut first_index = 0;
    let mut second_index = numbers.len() - 1;
    while first_index < second_index {
        let sum = numbers[first_index] + numbers[second_index];
        match sum.cmp(&needed_sum) {
            Ordering::Greater => second_index -= 1,
            Ordering::Less => first_index += 1,
            _ => return Some(numbers[first_index] * numbers[second_index]),
        }
    }
    None
}

#[aoc(day1, part2)]
fn day1_part2(input: &[i32]) -> Option<i32> {
    let mut numbers = vec![0; input.len()];
    numbers.clone_from_slice(input);
    numbers.sort_unstable();

    let needed_sum = 2020;
    for (first_index, first) in numbers.iter().enumerate() {
        let mut second_index = first_index + 1;
        let mut third_index = numbers.len() - 1;
        while second_index < third_index {
            let sum = first + numbers[second_index] + numbers[third_index];
            match sum.cmp(&needed_sum) {
                Ordering::Greater => third_index -= 1,
                Ordering::Less => second_index += 1,
                _ => return Some(first * numbers[second_index] * numbers[third_index]),
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(day1_part1(&input), Some(514579));
    }

    #[test]
    fn test_part2() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(day1_part2(&input), Some(241861950));
    }
}
