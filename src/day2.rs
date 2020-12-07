use aoc_runner_derive::{aoc, aoc_generator};

use regex::Regex;

#[aoc_generator(day2)]
fn parse_input_day2(input: &str) -> Result<Vec<(u32, u32, char, String)>, String> {
    let regex =
        Regex::new(r"(?P<minimum>\d+)-(?P<maximum>\d+) (?P<character>\w): (?P<password>\w+)")
            .unwrap();

    Ok(regex
        .captures_iter(input)
        .map(|cap| {
            (
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
                cap[3].parse().unwrap(),
                String::from(&cap[4]),
            )
        })
        .collect::<Vec<(u32, u32, char, String)>>())
}

#[aoc(day2, part1)]
fn day2_part1(input: &[(u32, u32, char, String)]) -> Option<i32> {
    let mut sum = 0;
    for record in input.iter() {
        let count = (record.3.len() - record.3.replace(record.2, "").len()) as u32;
        if count >= record.0 && count <= record.1 {
            sum += 1;
        }
    }
    Some(sum)
}

#[aoc(day2, part2)]
fn day2_part2(input: &[(u32, u32, char, String)]) -> Option<i32> {
    let mut sum = 0;
    for record in input.iter() {
        let chars = record.3.as_bytes();
        let st = chars[record.0 as usize - 1] as char;
        let nd = chars[record.1 as usize - 1] as char;

        if st == record.2 && nd == record.2 {
            continue;
        }

        if st == record.2 || nd == record.2 {
            sum += 1;
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
        assert_eq!(day2_part1(&parse_input_day2(input).unwrap()), Some(2));
    }

    #[test]
    fn test_part2() {
        let input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
        assert_eq!(day2_part2(&parse_input_day2(input).unwrap()), Some(1));
    }
}
