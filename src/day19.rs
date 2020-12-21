use regex::Regex;

use aoc_runner_derive::aoc;

const MAX_RULES: usize = 130;

fn parse_input_day19(input: &str) -> (Vec<String>, Vec<String>) {
    let mut input = input.split("\n\n");
    let rule_regex = Regex::new(r#"^(\d+): "?([\d\s\|\w]+)"?$"#).unwrap();
    let raw_rules: Vec<_> = input.next().unwrap().lines().collect();
    let mut rules = vec!["".to_string(); MAX_RULES];
    raw_rules.iter().for_each(|line| {
        let cap = rule_regex.captures_iter(line).next().unwrap();
        rules[cap[1].parse::<usize>().unwrap()] = cap[2].to_string();
    });

    let messages = input
        .next()
        .unwrap()
        .lines()
        .map(|line| line.to_owned())
        .collect::<Vec<_>>();

    (rules, messages)
}

fn calculate_regex_for_rule_p1(rules: &[String], index: &usize) -> String {
    let rule = &rules[*index];
    if rule.len() == 1 {
        format!(r#"({})"#, rule)
    } else {
        format!(
            r#"({})"#,
            rule.split('|')
                .map(|pat| {
                    format!(
                        r#"({})"#,
                        pat.trim()
                            .split(' ')
                            .map(|inner_rule| {
                                let index = inner_rule.parse::<usize>().unwrap();
                                calculate_regex_for_rule_p1(rules, &index)
                            })
                            .collect::<String>()
                    )
                })
                .collect::<Vec<_>>()
                .join("|")
        )
    }
}

fn calculate_regex_for_rule_p2(rules: &[String], index: &usize) -> String {
    let rule = &rules[*index];
    if rule.len() == 1 {
        format!(r#"({})"#, rule)
    } else if *index == 8 {
        // (8 = 42 | 42 8) == (42)+
        format!("(({})+)", calculate_regex_for_rule_p2(rules, &42))
    } else if *index == 11 {
        // (11 = 42 31 | 42 11 31) == (42){n}(31){n}
        // Since i didn't find a way to do this in regex
        // we will assume a maximum number of recursions
        // and keep trying to reach the number that passes.
        // Fortunately (5) was correct and worked from first trial :)
        let forty_two = calculate_regex_for_rule_p2(rules, &42);
        let thirty_one = calculate_regex_for_rule_p2(rules, &31);
        format!(
            "({}({}({}({}({}{})?{})?{})?{})?{})",
            forty_two,
            forty_two,
            forty_two,
            forty_two,
            forty_two,
            thirty_one,
            thirty_one,
            thirty_one,
            thirty_one,
            thirty_one
        )
    } else {
        format!(
            r#"({})"#,
            rule.split('|')
                .map(|pat| {
                    format!(
                        r#"({})"#,
                        pat.trim()
                            .split(' ')
                            .map(|inner_rule| {
                                let index = inner_rule.parse::<usize>().unwrap();
                                calculate_regex_for_rule_p2(rules, &index)
                            })
                            .collect::<String>()
                    )
                })
                .collect::<Vec<_>>()
                .join("|")
        )
    }
}

#[aoc(day19, part1)]
fn day19_part1(input: &str) -> Option<usize> {
    let (rules, messages) = parse_input_day19(input);
    let regex_0 = calculate_regex_for_rule_p1(&rules, &0);
    let rule_0_reg = Regex::new(&format!("^{}$", regex_0)).unwrap();
    messages
        .iter()
        .filter(|message| rule_0_reg.is_match(message))
        .count()
        .into()
}

#[aoc(day19, part2)]
fn day19_part2(input: &str) -> Option<usize> {
    let (mut rules, messages) = parse_input_day19(input);
    rules[8] = "42 | 42 8".to_string();
    rules[11] = "42 31 | 42 11 31".to_string();

    let regex_0 = calculate_regex_for_rule_p2(&rules, &0);
    let rule_0_reg = Regex::new(&format!("^{}$", regex_0)).unwrap();
    messages
        .iter()
        .filter(|message| rule_0_reg.is_match(message))
        .count()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "0: 4 1 5\n1: 2 3 | 3 2\n2: 4 4 | 5 5\n3: 4 5 | 5 4\n4: \"a\"\n5: \"b\"\n\nababbb\nbababa\nabbbab\naaabbb\naaaabbb";
        assert_eq!(day19_part1(input), Some(2));
    }

    #[test]
    fn test_part2() {
        let input = "42: 9 14 | 10 1\n9: 14 27 | 1 26\n10: 23 14 | 28 1\n1: \"a\"\n11: 42 31\n5: 1 14 | 15 1\n19: 14 1 | 14 14\n12: 24 14 | 19 1\n16: 15 1 | 14 14\n31: 14 17 | 1 13\n6: 14 14 | 1 14\n2: 1 24 | 14 4\n0: 8 11\n13: 14 3 | 1 12\n15: 1 | 14\n17: 14 2 | 1 7\n23: 25 1 | 22 14\n28: 16 1\n4: 1 1\n20: 14 14 | 1 15\n3: 5 14 | 16 1\n27: 1 6 | 14 18\n14: \"b\"\n21: 14 1 | 1 14\n25: 1 1 | 1 14\n22: 14 14\n8: 42\n26: 14 22 | 1 20\n18: 15 15\n7: 14 5 | 1 21\n24: 14 1\n\nabbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa\nbbabbbbaabaabba\nbabbbbaabbbbbabbbbbbaabaaabaaa\naaabbbbbbaaaabaababaabababbabaaabbababababaaa\nbbbbbbbaaaabbbbaaabbabaaa\nbbbababbbbaaaaaaaabbababaaababaabab\nababaaaaaabaaab\nababaaaaabbbaba\nbaabbaaaabbaaaababbaababb\nabbbbabbbbaaaababbbbbbaaaababb\naaaaabbaabaaaaababaa\naaaabbaaaabbaaa\naaaabbaabbaaaaaaabbbabbbaaabbaabaaa\nbabaaabbbaaabaababbaabababaaab\naabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";
        assert_eq!(day19_part2(input), Some(12));
    }
}

/*
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"


5: ^(b)$
4: ^(a)$
3: ^((ab)|(ba))$
2: ^((aa)|(bb))$
1: ^((((aa)|(bb))((ab)|(ba)))|(((ab)|(ba))((aa)|(bb))))$
0: ^((a)((((aa)|(bb))((ab)|(ba)))|(((ab)|(ba))((aa)|(bb))))(b))$
*/
