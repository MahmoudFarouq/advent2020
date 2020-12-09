use std::collections::HashMap;

use aoc_runner_derive::aoc;
use itertools::Itertools;
use regex::Regex;

#[aoc(day4, part1)]
fn day4_part1(input: &str) -> i32 {
    input
        .split("\n\n")
        .map(|passport_data| passport_data.replace('\n', " "))
        .map(|passport| match passport.split(' ').count() {
            8 => 1,
            7 => {
                if passport.contains("cid:") {
                    0
                } else {
                    1
                }
            }
            _ => 0,
        })
        .sum()
}

#[aoc(day4, part2)]
fn day4_part2(input: &str) -> i32 {
    let hcl_reg = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let allowed_ecl = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

    input
        .split("\n\n")
        .map(|passport_data| passport_data.replace('\n', " "))
        .map(|passport| {
            let map = passport
                .split(' ')
                .map(|field| field.splitn(2, ':').collect_tuple().unwrap())
                .collect::<HashMap<&str, &str>>();

            match map.get("byr") {
                None => return 0,
                Some(value) => {
                    if value.len() != 4 {
                        return 0;
                    }
                    let num = value.parse::<i32>().unwrap_or(0);
                    if num < 1920 || num > 2002 {
                        return 0;
                    }
                }
            }

            match map.get("iyr") {
                None => return 0,
                Some(value) => {
                    if value.len() != 4 {
                        return 0;
                    }
                    let num = value.parse::<i32>().unwrap_or(0);
                    if num < 2010 || num > 2020 {
                        return 0;
                    }
                }
            }

            match map.get("eyr") {
                None => return 0,
                Some(value) => {
                    if value.len() != 4 {
                        return 0;
                    }
                    let num = value.parse::<i32>().unwrap_or(0);
                    if num < 2020 || num > 2030 {
                        return 0;
                    }
                }
            }

            match map.get("hgt") {
                None => return 0,
                Some(value) => {
                    let v = value[..value.len() - 2].parse::<i32>().unwrap_or(0);
                    if value.ends_with("cm") {
                        if v < 150 || v > 193 {
                            return 0;
                        }
                    } else if value.ends_with("in") {
                        if v < 59 || v > 76 {
                            return 0;
                        }
                    } else {
                        return 0;
                    }
                }
            }

            match map.get("hcl") {
                None => return 0,
                Some(value) => {
                    if !hcl_reg.is_match(value) {
                        return 0;
                    }
                }
            }

            match map.get("ecl") {
                None => return 0,
                Some(value) => {
                    if !allowed_ecl.contains(value) {
                        return 0;
                    }
                }
            }

            match map.get("pid") {
                None => return 0,
                Some(value) => {
                    if value.len() != 9 || value.parse::<i32>().is_err() {
                        return 0;
                    }
                }
            }

            1
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

        assert_eq!(day4_part1(input), 2);
    }

    #[test]
    fn test_part2() {
        let input = "eyr:1972 cid:100
            hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

            iyr:2019
            hcl:#602927 eyr:1967 hgt:170cm
            ecl:grn pid:012533040 byr:1946

            hcl:dab227 iyr:2012
            ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

            hgt:59cm ecl:zzz
            eyr:2038 hcl:74454a iyr:2023
            pid:3556412378 byr:2007"
            .replace("            ", "");

        assert_eq!(day4_part2(&input), 0);
    }
}
