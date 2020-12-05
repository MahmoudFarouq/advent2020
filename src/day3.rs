use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
fn parse_input_day3(input: &str) -> Vec<Vec<u8>>{
    input
        .lines()
        .map(|line| {
            Vec::from(line.as_bytes())
        })
        .collect()
}

fn _solve_functional(input: &str, dx: usize, dy: usize) -> usize {
    input
        .lines()
        .step_by(dy)
        .zip(
            (0..input.find('\n').unwrap_or(input.len()))
                .cycle()
                .step_by(dx)
        )
        .filter(|(line, nth)| line.get(*nth..=*nth) == Some("#"))
        .count()
}

fn solve_imperative(input: &[Vec<u8>], dx: usize, dy: usize) -> usize {
    let mut x = 0;
    let mut y = 0;
    let width = input[0].len();

    let mut sum = 0;
    while y < input.len(){
        if input[y][x] as char == '#' {
            sum += 1;
        }
        x += dx;
        x %= width;

        y += dy;
    }
    sum
}

#[aoc(day3, part1)]
fn day3_part1(input: &[Vec<u8>]) -> usize {
    solve_imperative(input, 3, 1)
}


#[aoc(day3, part2)]
fn day3_part2(input: &[Vec<u8>]) -> usize {
    solve_imperative(input, 1, 1) * 
    solve_imperative(input, 3, 1) * 
    solve_imperative(input, 5, 1) * 
    solve_imperative(input, 7, 1) * 
    solve_imperative(input, 1, 2)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#";
        assert_eq!(day3_part1(&parse_input_day3(&input)), 7);
    }

    #[test]
    fn test_part2() {
        let input = "..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#";
        assert_eq!(day3_part2(&parse_input_day3(&input)), 336);
    }
}