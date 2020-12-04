use std::collections::HashMap;

use regex::Regex;
use aoc_runner_derive::{aoc};

#[aoc(day4, part1)]
fn day4_part1(input: &str) -> i32 {
    input
    .split("\n\n")
    .map(|passport_data| {
        passport_data
            .replace('\n', " ")
    })
    .map(|passport| {
        match passport
            .split(" ")
            .map(|field| field.split(":"))
            .count() {
            8 => 1,
            7 => if passport.contains("cid:") { 0 } else { 1 },
            _ => 0,
        }
    })
    .sum()
}

#[aoc(day4, part2)]
fn day4_part2(input: &str) -> i32 {
    let hcl_reg = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let allowed_ecl = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

    input
        .split("\n\n")
        .map(|passport_data| {
            passport_data
                .replace('\n', " ")
        })
        .map(|passport| {
            let mut map: HashMap<&str, &str> = HashMap::new();
            let _ = passport
                .split(" ")
                .map(|field| {
                    let res = field.split(":").collect::<Vec<&str>>();
                    map.insert(res[0], res[1]);
                }).collect::<()>();

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
                    let v = value[..value.len()-2].parse::<i32>().unwrap_or(0);
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