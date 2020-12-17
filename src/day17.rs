use std::{collections::HashMap, iter::FromIterator};
use aoc_runner_derive::aoc;

#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    Active,
    Inactive
}

impl Default for State {
    fn default() -> Self {
        State::Inactive
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
    w: isize,
}

impl Point {
    fn new(x: isize, y: isize, z: isize, w: isize) -> Self {
        Point { x, y, z, w }
    }

    fn neighbors(&self) -> Vec<Point> {
        let mut vec = Vec::with_capacity(26);
        for x in (-1..=1).into_iter() {
            for y in (-1..=1).into_iter() {
                for z in (-1..=1).into_iter() {
                    for w in (-1..=1).into_iter() {
                        if x == 0 && y == 0 && z == 0 && w == 0 {
                            continue
                        }
                        vec.push(Point::new(self.x + x, self.y + y, self.z + z, self.w + w));
                    }
                }
            }
        }
        vec
    }
}

#[derive(Debug, Default)]
struct NeighborsCount {
    active: usize,
    inactive: usize,
}

#[derive(Default, Debug)]
struct Grid {
    grid: HashMap<Point, State>,
}

impl Grid {
    fn set(&mut self, point: &Point, state: State) {
        *self.grid.entry(point.clone()).or_default() = state;
    }

    fn run(&mut self, n: usize) {
        for _ in 0..n {
            self.step();
        }
    }

    fn step(&mut self) {
        let mut new_grid = self.grid.clone();
        self.build_new_layer(&mut new_grid);
        self.grid
            .iter()
            .for_each(|(point, state)| {
                let counts = self.get_neighbors_states_count(point);

                if *state == State::Active && counts.active != 2 && counts.active != 3 {
                    *new_grid.entry(point.clone()).or_default() = State::Inactive;
                }

                if *state == State::Inactive && counts.active == 3 {
                    *new_grid.entry(point.clone()).or_default() = State::Active;
                }

            });

        self.grid = new_grid;
    }

    fn build_new_layer(&mut self, new_board: &HashMap<Point, State>) {
        new_board
            .iter()
            .for_each(|(point, _)| {
                point
                    .neighbors()
                    .iter()
                    .for_each(|neighbor| {
                        self.grid.entry(neighbor.clone()).or_default();
                    });
            });
    }

    fn get_neighbors_states_count(&self, point: &Point) -> NeighborsCount {
        let mut count = NeighborsCount::default();
        point
            .neighbors()
            .iter()
            .for_each(|neighbor| {
                match self.grid.get(neighbor).unwrap_or(&State::Inactive) {
                    State::Active => count.active += 1,
                    State::Inactive => count.inactive += 1,
                }
            });
        count
    }

    fn count_alive(&self) -> usize {
        self.grid
            .iter()
            .filter(|&(_, state)| *state == State::Active)
            .count()
    }
}

impl FromIterator<Vec<State>> for Grid {
    fn from_iter<T: IntoIterator<Item = Vec<State>>>(iter: T) -> Self {
        let initial = iter.into_iter().collect::<Vec<_>>();

        let mut grid = Self::default();
        initial
            .iter()
            .enumerate()
            .for_each(|(x, row)| {
                row
                    .iter()
                    .enumerate()
                    .for_each(|(y, cell)| {
                        let point = Point::new(x as isize, y as isize, 0, 0);
                        grid.set(&point, *cell);
                    })
            });

        grid
    }
}

fn parse_input_day17(input: &str) -> Grid {
    input
        .lines()
        .map(|line| {
            line
                .chars()
                .map(|cell| {
                    match cell {
                        '#' => State::Active,
                        '.' => State::Inactive,
                        _ => unreachable!()
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

#[aoc(day17, part1)]
fn day17_part1(input: &str) -> Option<usize> {
    let mut grid = parse_input_day17(input);
    grid.run(6);
    Some(grid.count_alive())
}

#[aoc(day17, part2)]
fn day17_part2(input: &str) -> Option<usize> {
    let mut grid = parse_input_day17(input);
    grid.run(6);
    Some(grid.count_alive())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = ".#.\n..#\n###";
        assert_eq!(day17_part1(input), Some(112));
    }

    #[test]
    fn test_part2() {
        let input = ".#.\n..#\n###";
        assert_eq!(day17_part2(input), Some(848));
    }
}
