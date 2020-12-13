use aoc_runner_derive::{aoc, aoc_generator};

struct Notes {
    timestamp: usize,
    ids: Vec<(usize, usize)>,
}

#[aoc_generator(day13)]
fn parse_input_day13(input: &str) -> Notes {
    let mut splits = input.splitn(2, '\n');
    let timestamp = splits.next().unwrap().parse().unwrap();
    let ids = splits
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|&(_, id)| id != "x")
        .map(|(index, id)| (index as usize, id.parse().unwrap()))
        .collect();

    Notes { timestamp, ids }
}

#[aoc(day13, part1)]
fn day13_part1(notes: &Notes) -> Option<usize> {
    let mut best_diff = usize::MAX;
    let mut best_id = 0;
    for (_, id) in notes.ids.iter() {
        let new_diff = id - notes.timestamp % id;
        if new_diff < best_diff {
            best_diff = new_diff;
            best_id = *id;
        }
    }
    Some(best_id * best_diff)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "939\n7,13,x,x,59,x,31,19";
        assert_eq!(day13_part1(&parse_input_day13(input)), Some(295));
    }
}
