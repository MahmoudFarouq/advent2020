use aoc_runner_derive::{aoc, aoc_generator};

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Edge {
    Top,
    Right,
    Down,
    Left,
}

impl Edge {
    fn new(i: usize) -> Self {
        Self::from(i)
    }

    fn as_usize(&self) -> usize {
        match *self {
            Edge::Top => 0,
            Edge::Right => 1,
            Edge::Down => 2,
            Edge::Left => 3,
        }
    }
}

impl From<usize> for Edge {
    fn from(i: usize) -> Self {
        match i {
            0 => Self::Top,
            1 => Self::Right,
            2 => Self::Down,
            3 => Self::Left,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Tile {
    id: usize,
    edges: Vec<String>,
}

impl Tile {
    fn new(id: usize, edges: Vec<String>) -> Self {
        Self { id, edges }
    }
}

#[aoc_generator(day20)]
fn parse_input_day20(input: &str) -> Vec<Tile> {
    input
        .split("\n\n")
        .map(|tile| {
            let lines: Vec<_> = tile.lines().collect();
            let id = lines[0][5..lines[0].len() - 1].parse::<usize>().unwrap();

            let mut edges = Vec::with_capacity(4);
            // Top Edge
            edges.push(lines[1].to_string());

            // Right Edge
            edges.push(
                lines[1..]
                    .iter()
                    .map(|&line| line.chars().last().unwrap())
                    .collect(),
            );

            // Bottom Edge
            edges.push(lines[lines.len() - 1].chars().rev().collect());

            // Left Edge
            edges.push(
                lines[1..]
                    .iter()
                    .map(|&line| line.chars().next().unwrap())
                    .rev()
                    .collect(),
            );

            Tile::new(id, edges)
        })
        .collect()
}

#[aoc(day20, part1)]
fn day20_part1(tiles: &[Tile]) -> Option<usize> {
    // ..## => (id, left)
    let mut hasher = HashMap::<&String, (usize, Edge)>::new();
    let mut adjacent_list = tiles
        .iter()
        .map(|tile| (tile.id, vec![0; 4]))
        .collect::<HashMap<_, _>>();

    for tile in tiles.iter() {
        for (i, edge) in tile.edges.iter().enumerate() {
            let dir = Edge::new(i);
            let reverse = edge.chars().rev().collect();
            let candidate = if hasher.contains_key(edge) {
                edge
            } else if hasher.contains_key(&reverse) {
                &reverse
            } else {
                hasher.insert(edge, (tile.id, dir));
                continue;
            };

            let (neighbor_id, neighbor_edge) = hasher.get(candidate).unwrap();
            // If we are not another side of the same tile that is like this one
            // and our directions are opposites, then we are connected.
            if *neighbor_id != tile.id {
                adjacent_list.entry(tile.id).or_default()[dir.as_usize()] = *neighbor_id;
                adjacent_list.entry(*neighbor_id).or_default()[neighbor_edge.as_usize()] = tile.id;
            }
        }
    }

    // Multiply only those who have only 2 edges filled
    adjacent_list
        .iter()
        .filter_map(|(id, connectors)| {
            if connectors
                .iter()
                .filter(|&connector| *connector != 0)
                .count()
                == 2
            {
                Some(id)
            } else {
                None
            }
        })
        .fold(1, |acc, id| id * acc)
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Tile 2311:\n..##.#..#.\n##..#.....\n#...##..#.\n####.#...#\n##.##.###.\n##...#.###\n.#.#.#..##\n..#....#..\n###...#.#.\n..###..###\n\nTile 1951:\n#.##...##.\n#.####...#\n.....#..##\n#...######\n.##.#....#\n.###.#####\n###.##.##.\n.###....#.\n..#.#..#.#\n#...##.#..\n\nTile 1171:\n####...##.\n#..##.#..#\n##.#..#.#.\n.###.####.\n..###.####\n.##....##.\n.#...####.\n#.##.####.\n####..#...\n.....##...\n\nTile 1427:\n###.##.#..\n.#..#.##..\n.#.##.#..#\n#.#.#.##.#\n....#...##\n...##..##.\n...#.#####\n.#.####.#.\n..#..###.#\n..##.#..#.\n\nTile 1489:\n##.#.#....\n..##...#..\n.##..##...\n..#...#...\n#####...#.\n#..#.#.#.#\n...#.#.#..\n##.#...##.\n..##.##.##\n###.##.#..\n\nTile 2473:\n#....####.\n#..#.##...\n#.##..#...\n######.#.#\n.#...#.#.#\n.#########\n.###.#..#.\n########.#\n##...##.#.\n..###.#.#.\n\nTile 2971:\n..#.#....#\n#...###...\n#.#.###...\n##.##..#..\n.#####..##\n.#..####.#\n#..#.#..#.\n..####.###\n..#.#.###.\n...#.#.#.#\n\nTile 2729:\n...#.#.#.#\n####.#....\n..#.#.....\n....#..#.#\n.##..##.#.\n.#.####...\n####.#.#..\n##.####...\n##..#.##..\n#.##...##.\n\nTile 3079:\n#.#.#####.\n.#..######\n..#.......\n######....\n####.#..#.\n.#...#.##.\n#.#####.##\n..#.###...\n..#.......\n..#.###...";
        assert_eq!(day20_part1(&parse_input_day20(input)), Some(20899048083289));
    }
}
