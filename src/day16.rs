use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
enum Rule {
    InRange(usize, usize),
}

#[derive(Debug)]
struct Field {
    name: String,
    rules: Vec<Rule>,
}

impl Field {
    fn validate(&self, value: usize) -> bool {
        self.rules
            .iter()
            .map(|rule| match rule {
                Rule::InRange(min, max) => value >= *min && value <= *max,
            })
            .any(|e| e)
    }
}

struct Input {
    fields: Vec<Field>,
    personal_ticket: Vec<usize>,
    nearby_tickets: Vec<Vec<usize>>,
}

impl Input {
    fn validate_ticket(&self, index: usize) -> bool {
        self.nearby_tickets[index]
            .iter()
            .map(|value| {
                self.fields
                    .iter()
                    .map(|field| field.validate(*value))
                    .any(|e| e)
            })
            .all(|e| e)
    }
}

#[aoc_generator(day16)]
fn parse_input_day16(input: &str) -> Input {
    let mut sections = input.split("\n\n");
    let fields = sections
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");

            Field {
                name: parts.next().unwrap().to_string(),
                rules: parts
                    .next()
                    .unwrap()
                    .split(" or ")
                    .map(|range| {
                        let mut bounds = range.split('-');
                        Rule::InRange(
                            bounds.next().unwrap().parse().unwrap(),
                            bounds.next().unwrap().parse().unwrap(),
                        )
                    })
                    .collect(),
            }
        })
        .collect();

    let personal_ticket = sections
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|line| {
            line.split(',')
                .map(|value| value.parse().unwrap())
                .collect()
        })
        .last()
        .unwrap();

    let nearby_tickets = sections
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|line| {
            line.split(',')
                .map(|value| value.parse().unwrap())
                .collect()
        })
        .collect();

    Input {
        fields,
        personal_ticket,
        nearby_tickets,
    }
}

#[aoc(day16, part1)]
fn day16_part1(input: &Input) -> Option<usize> {
    input
        .nearby_tickets
        .iter()
        .map(|ticket| {
            ticket
                .iter()
                .filter_map(|value| {
                    for field in input.fields.iter() {
                        for rule in field.rules.iter() {
                            match rule {
                                Rule::InRange(min, max) => {
                                    if value >= min && value <= max {
                                        return None;
                                    }
                                }
                            }
                        }
                    }
                    Some(*value)
                })
                .sum::<usize>()
        })
        .sum::<usize>()
        .into()
}

#[aoc(day16, part2)]
fn day16_part2(input: &Input) -> Option<usize> {
    let valid_tickets = input
        .nearby_tickets
        .iter()
        .enumerate()
        .filter_map(|(index, ticket)| {
            if input.validate_ticket(index) {
                Some(ticket)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut last_solutions = Vec::with_capacity(valid_tickets[0].len());
    let mut found_fields = Vec::with_capacity(valid_tickets[0].len());
    let mut indexes: Vec<usize> = (0..valid_tickets[0].len()).collect();

    while !indexes.is_empty() {
        let column_index = indexes.remove(0);

        let mut column: Vec<_> = valid_tickets
            .iter()
            .map(|&ticket| ticket[column_index])
            .collect();
        column.sort_unstable();
        let mut solutions = Vec::new();

        'fields: for field in input
            .fields
            .iter()
            .filter(|&field| !found_fields.contains(&field.name))
        {
            for value in column.iter() {
                if field.validate(*value) {
                    continue;
                } else {
                    continue 'fields;
                }
            }
            solutions.push((column_index, field.name.clone()));
        }
        if solutions.len() == 1 {
            last_solutions.push(solutions[0].clone());
            found_fields.push(solutions[0].1.clone());
        } else {
            // Try later when there is only one solution
            indexes.push(column_index);
        }
    }

    last_solutions
        .iter()
        .filter(|(_, name)| name.starts_with("departure"))
        .fold(1, |acc, (i, _)| input.personal_ticket[*i] * acc)
        .into()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "class: 1-3 or 5-7\nrow: 6-11 or 33-44\nseat: 13-40 or 45-50\n\nyour ticket:\n7,1,14\n\nnearby tickets:\n7,3,47\n40,4,50\n55,2,20\n38,6,12";
        assert_eq!(day16_part1(&parse_input_day16(input)), Some(71));
    }
}
