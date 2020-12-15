use std::collections::HashMap;

use regex::Regex;

use aoc_runner_derive::aoc;

#[derive(Debug)]
enum InputLine {
    Mask(String),
    Write(usize, usize),
}

fn parse_input_day14(input: &str) -> Vec<InputLine> {
    let mask_reg = Regex::new(r"mask = (?P<mask>[X10]+)").unwrap();
    let write_reg = Regex::new(r"mem\[(?P<address>\d+)\] = (?P<value>\d+)").unwrap();
    input
        .lines()
        .map(|line| {
            if mask_reg.is_match(line) {
                InputLine::Mask(mask_reg.captures_iter(line).last().unwrap()[1].to_owned())
            } else if write_reg.is_match(line) {
                let cap = write_reg.captures_iter(line).last().unwrap();
                InputLine::Write(cap[1].parse().unwrap(), cap[2].parse().unwrap())
            } else {
                unreachable!()
            }
        })
        .collect()
}

#[aoc(day14, part1)]
fn day14_part1(input: &str) -> Option<usize> {
    let lines = parse_input_day14(input);
    let mut memory: HashMap<usize, usize> = HashMap::new();
    let mut current_mask = &"".to_owned();
    for line in lines.iter() {
        match line {
            InputLine::Mask(mask) => {
                current_mask = mask;
            }
            InputLine::Write(address, value) => {
                let mut binary: Vec<_> = format!("{:036b}", value).chars().collect();
                current_mask
                    .chars()
                    .enumerate()
                    .map(|(i, bit)| {
                        if bit != 'X' {
                            binary[i] = bit;
                        }
                    })
                    .for_each(drop);
                *memory.entry(*address).or_default() =
                    usize::from_str_radix(&binary.iter().collect::<String>(), 2).unwrap();
            }
        }
    }

    memory.values().sum::<usize>().into()
}

#[aoc(day14, part2)]
fn day14_part2(input: &str) -> Option<usize> {
    let lines = parse_input_day14(input);
    let mut memory: HashMap<usize, usize> = HashMap::new();
    let mut current_mask = &"".to_owned();
    for line in lines.iter() {
        match line {
            InputLine::Mask(mask) => {
                current_mask = mask;
            }
            InputLine::Write(address, value) => {
                let mut binary: Vec<_> = format!("{:036b}", address).chars().collect();
                binary = current_mask
                    .chars()
                    .enumerate()
                    .map(|(i, bit)| {
                        if bit == '1' || bit == 'X' {
                            bit
                        } else {
                            binary[i]
                        }
                    })
                    .collect();

                let mut combinations = Vec::new();
                combinations.push(binary);
                while !combinations.is_empty() {
                    let mut flag = true;
                    let current = combinations.pop().unwrap();
                    for (i, bit) in current.iter().enumerate() {
                        if *bit == 'X' {
                            let mut _1 = current.clone();
                            _1[i] = '1';
                            let mut _0 = current.clone();
                            _0[i] = '0';
                            combinations.push(_1);
                            combinations.push(_0);
                            flag = false;
                            break;
                        }
                    }
                    if flag {
                        *memory
                            .entry(
                                usize::from_str_radix(&current.iter().collect::<String>(), 2)
                                    .unwrap(),
                            )
                            .or_default() = *value;
                    }
                }
            }
        }
    }

    memory.values().sum::<usize>().into()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input =
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\nmem[8] = 11\nmem[7] = 101\nmem[8] = 0";
        assert_eq!(day14_part1(input), Some(165))
    }

    #[test]
    fn test_part2() {
        let input = "mask = 000000000000000000000000000000X1001X\nmem[42] = 100\nmask = 00000000000000000000000000000000X0XX\nmem[26] = 1";
        assert_eq!(day14_part2(input), Some(208))
    }
}
