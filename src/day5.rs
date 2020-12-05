use aoc_runner_derive::{aoc};

fn code_to_id(code: &str) -> usize {
    let binary = code
        .replace('F', "0")
        .replace('B', "1")
        .replace('L', "0")
        .replace('R', "1");
    usize::from_str_radix(&binary, 2).unwrap_or(0)
}

#[aoc(day5, part1)]
fn day5_part1(input: &str) -> Option<usize> {
    input
        .lines()
        .map(|line|{
            code_to_id(&line[..7]) * 8 + code_to_id(&line[7..])
        })
        .max()
}

#[aoc(day5, part2)]
fn day5_part2(input: &str) -> Option<usize> {
    let mut ids = input
        .lines()
        .map(|line|{
            code_to_id(&line[..7]) * 8 + code_to_id(&line[7..])
        })
        .collect::<Vec<usize>>();
    
    ids.sort();
    for window in ids.windows(2) {
        if window[0] == window[1] - 2 {
            return Some(window[0] + 1);
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