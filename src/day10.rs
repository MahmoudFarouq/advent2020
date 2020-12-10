use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day10)]
fn parse_input_day10(input: &str) -> Vec<usize> {
    let mut rates = input
        .lines()
        .map(|rate| rate.parse().unwrap())
        .collect::<Vec<usize>>();
    rates.sort_unstable();

    rates
}

#[aoc(day10, part1)]
fn day10_part1(rates: &[usize]) -> Option<usize> {
    let mut record = vec![0, 0, 1]; // ones, twos, threes
    rates.iter().fold(0, |acc, &node| {
        record[node - acc - 1] += 1;
        node
    });
    Some(record[0] * record[2])
}

#[aoc(day10, part2)]
fn day10_part2(rates: &[usize]) -> Option<usize> {
    let mut rates = Vec::from(rates);
    rates.push(rates.last().unwrap().clone() + 3); // Phone
    rates.insert(0, 0); // charging outlet

    let mut memo = vec![0; rates.len()];
    memo[rates.len() - 1] = 1; // There is only one way from last to itself

    rates
        .iter()
        .enumerate()
        .rev()
        .skip(1)
        .map(|(index, &node)| {
            let start = index + 1;
            let end = start + 3.min(rates.len() - index - 1);
            let candidates_count = &rates[start..end]
                .iter()
                .filter(|&&candidate| (candidate as isize - node as isize) <= 3)
                .count();

            memo[index] = if *candidates_count == 1 {
                memo[start]
            } else {
                memo[start..start + candidates_count].iter().sum()
            }
        })
        .for_each(drop);

    Some(memo[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4";
        assert_eq!(day10_part1(&parse_input_day10(input)), Some(35));

        let input = "28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3";
        assert_eq!(day10_part1(&parse_input_day10(input)), Some(220));
    }

    #[test]
    fn test_part2() {
        let input = "16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4";
        assert_eq!(day10_part2(&parse_input_day10(input)), Some(8));

        let input = "28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3";
        assert_eq!(day10_part2(&parse_input_day10(input)), Some(19208));
    }
}

/*
0       1   4    5      6     7     10     11   12       15     16     19   22
                                                                            1
                                                                        1
                                                                1
                                                        1
                                                1
                                            1
                                    2
*/
