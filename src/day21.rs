use aoc_runner_derive::aoc;

use std::collections::{HashMap, HashSet};

fn parse_input_day21(input: &str) -> (HashMap<&str, usize>, HashMap<&str, &str>) {
    // Allergen => [Ingredients]
    let mut one_to_many = HashMap::new();
    let mut occurrences = HashMap::new();
    input.lines().for_each(|line| {
        let mut split = line.split(" (contains ");
        let ingredients = split.next().unwrap().split(' ').collect::<HashSet<_>>();

        let allergens = split.next().unwrap();
        let allergens: Vec<_> = (&allergens[..allergens.len() - 1]).split(", ").collect();

        for &allergen in allergens.iter() {
            let set = one_to_many
                .entry(allergen)
                .or_insert_with(|| ingredients.clone());
            *set = set.intersection(&ingredients).copied().collect();
        }

        for ingredient in ingredients.iter() {
            *occurrences.entry(*ingredient).or_default() += 1;
        }
    });

    // Allergen => Ingredient
    let mut one_to_one = HashMap::new();
    while let Some((&allergen, ingredients)) = one_to_many.iter().find(|(_, set)| set.len() == 1) {
        let ing = *ingredients.iter().next().unwrap();
        one_to_one.insert(allergen, ing);
        one_to_many.iter_mut().for_each(|(_, set)| {
            set.remove(ing);
        });
    }

    (occurrences, one_to_one)
}

#[aoc(day21, part1)]
fn day21_part1(input: &str) -> usize {
    let (occurrences, one_to_one) = parse_input_day21(input);
    occurrences
        .iter()
        .filter(|(&allergen, _)| {
            one_to_one
                .values()
                .find(|&&some| some == allergen)
                .is_none()
        })
        .map(|(&_, count)| count)
        .sum()
}

#[aoc(day21, part2)]
fn day21_part2(input: &str) -> String {
    let (_, one_to_one) = parse_input_day21(input);
    let mut res = one_to_one.keys().copied().collect::<Vec<_>>();
    res.sort_unstable();

    res.iter()
        .map(|&key| *one_to_one.get(key).unwrap())
        .collect::<Vec<_>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)\ntrh fvjkl sbzzf mxmxvkd (contains dairy)\nsqjhc fvjkl (contains soy)\nsqjhc mxmxvkd sbzzf (contains fish)";
        assert_eq!(day21_part1(input), 5);
    }

    #[test]
    fn test_part1() {
        let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)\ntrh fvjkl sbzzf mxmxvkd (contains dairy)\nsqjhc fvjkl (contains soy)\nsqjhc mxmxvkd sbzzf (contains fish)";
        assert_eq!(day21_part2(input), "mxmxvkd,sqjhc,fvjkl");
    }
}
