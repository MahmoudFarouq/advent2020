use std::collections::{HashSet, VecDeque};

use aoc_runner_derive::aoc;

enum Winner {
    PlayerOne,
    PlayerTwo,
}

fn parse_input_day22(input: &str) -> Vec<VecDeque<usize>> {
    input
        .split("\n\n")
        .map(|deck| {
            deck.lines()
                .skip(1)
                .map(|card| card.parse::<usize>().unwrap())
                .collect()
        })
        .collect()
}

#[aoc(day22, part1)]
fn day22_part1(input: &str) -> Option<usize> {
    let mut players = parse_input_day22(input);
    let mut player_2 = players.pop().unwrap();
    let mut player_1 = players.pop().unwrap();

    while !player_1.is_empty() && !player_2.is_empty() {
        let p1_card = player_1.pop_front().unwrap();
        let p2_card = player_2.pop_front().unwrap();

        if p1_card > p2_card {
            player_1.push_back(p1_card);
            player_1.push_back(p2_card);
        } else {
            player_2.push_back(p2_card);
            player_2.push_back(p1_card);
        }
    }

    let winner = if player_1.is_empty() {
        player_2
    } else {
        player_1
    };

    winner
        .iter()
        .rev()
        .zip(1..)
        .fold(0, |acc, (index, card)| acc + index * card)
        .into()
}

#[aoc(day22, part2)]
fn day22_part2(input: &str) -> Option<usize> {
    let mut players = parse_input_day22(input);
    let mut player_2 = players.pop().unwrap();
    let mut player_1 = players.pop().unwrap();

    let winner = match sub_game(&mut player_1, &mut player_2) {
        Winner::PlayerOne => player_1,
        Winner::PlayerTwo => player_2,
    };

    winner
        .iter()
        .rev()
        .zip(1..)
        .fold(0, |acc, (index, card)| acc + index * card)
        .into()
}

fn sub_game(player_1: &mut VecDeque<usize>, player_2: &mut VecDeque<usize>) -> Winner {
    let mut memo = HashSet::new();

    while !player_1.is_empty() && !player_2.is_empty() {
        let clones = (player_1.clone(), player_2.clone());
        if memo.contains(&clones) {
            return Winner::PlayerOne;
        }
        memo.insert(clones);

        let p1_card = player_1.pop_front().unwrap();
        let p2_card = player_2.pop_front().unwrap();

        if player_1.len() >= p1_card && player_2.len() >= p2_card {
            let mut p1_new_deck = player_1.iter().take(p1_card).copied().collect();
            let mut p2_new_deck = player_2.iter().take(p2_card).copied().collect();
            match sub_game(&mut p1_new_deck, &mut p2_new_deck) {
                Winner::PlayerOne => {
                    player_1.push_back(p1_card);
                    player_1.push_back(p2_card);
                }
                Winner::PlayerTwo => {
                    player_2.push_back(p2_card);
                    player_2.push_back(p1_card);
                }
            }
        } else {
            if p1_card > p2_card {
                player_1.push_back(p1_card);
                player_1.push_back(p2_card);
            } else {
                player_2.push_back(p2_card);
                player_2.push_back(p1_card);
            }
        }
    }

    if player_1.is_empty() {
        Winner::PlayerTwo
    } else {
        Winner::PlayerOne
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Player 1:\n9\n2\n6\n3\n1\n\nPlayer 2:\n5\n8\n4\n7\n10";
        assert_eq!(day22_part1(input), Some(306));
    }

    #[test]
    fn test_part2() {
        let input = "Player 1:\n9\n2\n6\n3\n1\n\nPlayer 2:\n5\n8\n4\n7\n10";
        assert_eq!(day22_part2(input), Some(291));
    }
}
