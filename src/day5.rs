use aoc_runner_derive::{aoc};

fn split(range: (i32, i32), c: char) -> (i32, i32) {
    let half = (range.1 - range.0) / 2 + 1;
    match c {
        'F' | 'L' => (range.0       , range.1 - half),
        'B' | 'R' => (range.0 + half, range.1       ),
        _ => unreachable!()
    }
}

#[aoc(day5, part1)]
fn day5_part1(input: &str) -> Option<i32> {
    input
        .lines()
        .map(|line|{
            line[..7]
                .chars()
                .fold((0, 127), split).0 * 8 + 
            line[7..]
                .chars()
                .fold((0, 7), split).0
        })
        .max()
}

#[aoc(day5, part2)]
fn day5_part2(input: &str) -> Option<i32> {
    let mut ids = input
        .lines()
        .map(|line|{
            line[..7]
                .chars()
                .fold((0, 127), split).0 * 8 + 
            line[7..]
                .chars()
                .fold((0, 7), split).0
        })
        .collect::<Vec<i32>>();
    
    ids.sort();
    for (i, _) in ids[..ids.len()-1].iter().enumerate() {
        if ids[i] == ids[i+1] - 2 {
            return Some(ids[i] + 1);
        } 
    }

    None
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(day5_part1("FBFBBFFRLR"), Some(357));
        assert_eq!(day5_part1("BFFFBBFRRR"), Some(567));
        assert_eq!(day5_part1("FFFBBBFRRR"), Some(119));
        assert_eq!(day5_part1("BBFFBBFRLL"), Some(820));
    }

}