use aoc_runner_derive::aoc;

static DIRECTIONS: &[(isize, isize)] = &[
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, -1),
    (1, 1),
    (-1, 1),
    (-1, -1),
];

struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Point { x, y }
    }
    fn step_t(&mut self, dxdy: &(isize, isize)) {
        self.x += dxdy.0;
        self.y += dxdy.1;
    }
    fn is_valid(&self, max_x: isize, max_y: isize) -> bool {
        !(self.x < 0 || self.x >= max_x || self.y < 0 || self.y >= max_y)
    }
}

#[derive(Debug, Default)]
struct AdjacentCount {
    floor: usize,
    empty: usize,
    occupied: usize,
}

enum AdjacentStrategy {
    ClosestEight,
    FirstSeenInEachDirection,
}

impl AdjacentStrategy {
    fn get_adjacent_seats(&self, board: &[Vec<char>], row: usize, column: usize) -> Vec<Point> {
        match self {
            Self::ClosestEight => self.get_points_1(board, row, column),
            Self::FirstSeenInEachDirection => self.get_points_2(board, row, column),
        }
    }

    fn get_points_1(&self, board: &[Vec<char>], row: usize, column: usize) -> Vec<Point> {
        let mut result = Vec::new();
        let max_x = board.len() as isize;
        let max_y = board.first().unwrap().len() as isize;

        for change in DIRECTIONS.iter() {
            let mut point = Point::new(row as isize, column as isize);
            point.step_t(change);
            if !point.is_valid(max_x, max_y) {
                continue;
            }
            result.push(point);
        }
        result
    }

    fn get_points_2(&self, board: &[Vec<char>], row: usize, column: usize) -> Vec<Point> {
        let mut result = Vec::new();
        let max_x = board.len() as isize;
        let max_y = board.first().unwrap().len() as isize;

        for change in DIRECTIONS.iter() {
            let mut point = Point::new(row as isize, column as isize);
            loop {
                point.step_t(change);
                if !point.is_valid(max_x, max_y) {
                    break;
                }
                match board[point.x as usize][point.y as usize] {
                    '.' => continue,
                    _ => {
                        result.push(point);
                        break;
                    }
                }
            }
        }
        result
    }
}

struct SimulationRules {
    max_occupied_to_sit: usize,
    min_occupied_to_leave: usize,
    adjacent_strategy: AdjacentStrategy,
}

impl SimulationRules {
    fn new(
        max_occupied_to_sit: usize,
        min_occupied_to_leave: usize,
        adjacent_strategy: AdjacentStrategy,
    ) -> Self {
        SimulationRules {
            max_occupied_to_sit,
            min_occupied_to_leave,
            adjacent_strategy,
        }
    }
}

struct SimulationStep {
    board: Vec<Vec<char>>,
    changes: usize,
}

impl SimulationStep {
    fn new(board: Vec<Vec<char>>) -> Self {
        SimulationStep { board, changes: 0 }
    }
}

struct Simulator {
    rules: SimulationRules,
    step: SimulationStep,
}

impl Simulator {
    fn new(rules: SimulationRules, initial_step: SimulationStep) -> Self {
        Self {
            rules,
            step: initial_step,
        }
    }

    fn run_to_end(&mut self) {
        loop {
            self.step = self.next_step();
            if self.step.changes == 0 {
                break;
            }
        }
    }

    fn count_adjacent_seats(&self, row: usize, column: usize) -> AdjacentCount {
        let mut counts = AdjacentCount::default();
        let adjacent_seats =
            self.rules
                .adjacent_strategy
                .get_adjacent_seats(&self.step.board, row, column);
        for seat in adjacent_seats.iter() {
            match self.step.board[seat.x as usize][seat.y as usize] {
                'L' => counts.empty += 1,
                '#' => counts.occupied += 1,
                '.' => counts.floor += 1,
                _ => unreachable!(),
            }
        }
        counts
    }

    fn next_step(&self) -> SimulationStep {
        let mut new_step = SimulationStep::new(self.step.board.clone());
        let rows = self.step.board.len();
        let columns = self.step.board.first().unwrap().len();
        for row in 0..rows {
            for column in 0..columns {
                let seats_count = self.count_adjacent_seats(row, column);
                if self.step.board[row][column] == 'L'
                    && seats_count.occupied == self.rules.max_occupied_to_sit
                {
                    new_step.board[row][column] = '#';
                    new_step.changes += 1;
                } else if self.step.board[row][column] == '#'
                    && seats_count.occupied >= self.rules.min_occupied_to_leave
                {
                    new_step.board[row][column] = 'L';
                    new_step.changes += 1;
                }
            }
        }
        new_step
    }

    fn count_occupied(&self) -> usize {
        self.step
            .board
            .iter()
            .map(|row| row.iter().filter(|&&seat| seat == '#').count())
            .sum()
    }
}

fn parse_input_day11(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[aoc(day11, part1)]
fn day11_part1(input: &str) -> Option<usize> {
    let board = parse_input_day11(input);
    let rules = SimulationRules::new(0, 4, AdjacentStrategy::ClosestEight);
    let mut simulator = Simulator::new(rules, SimulationStep::new(board));
    simulator.run_to_end();

    Some(simulator.count_occupied())
}

#[aoc(day11, part2)]
fn day11_part2(input: &str) -> Option<usize> {
    let board = parse_input_day11(input);
    let rules = SimulationRules::new(0, 5, AdjacentStrategy::FirstSeenInEachDirection);
    let mut simulator = Simulator::new(rules, SimulationStep::new(board));
    simulator.run_to_end();

    Some(simulator.count_occupied())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL";
        assert_eq!(day11_part1(input), Some(37));
    }

    #[test]
    fn test_part2() {
        let input = "L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL";
        assert_eq!(day11_part2(input), Some(26));
    }
}
