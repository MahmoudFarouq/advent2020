use aoc_runner_derive::aoc;

#[derive(Debug, Copy, Clone)]
enum Token {
    Add,
    Multiply,
    LeftPrentices,
    RightPrentices,
    Number(usize),
}

fn parse_input_day18(input: &str) -> Vec<Vec<Token>> {
    input
        .lines()
        .map(|line| {
            line.replace(" ", "")
                .chars()
                .map(|token| match token {
                    '+' => Token::Add,
                    '*' => Token::Multiply,
                    '(' => Token::LeftPrentices,
                    ')' => Token::RightPrentices,
                    number => Token::Number(number.to_string().parse().unwrap()),
                })
                .collect()
        })
        .collect()
}

trait Solver {
    fn evaluate(&self, expr: &mut Vec<&Token>) -> usize;
    fn factor(&self, expr: &mut Vec<&Token>) -> usize;
    fn term(&self, _: &mut Vec<&Token>) -> usize {
        unreachable!();
    }
}

struct SolverPartOne;
impl Solver for SolverPartOne {
    fn evaluate(&self, mut expr: &mut Vec<&Token>) -> usize {
        let mut result = self.factor(&mut expr);
        while !expr.is_empty() {
            match expr.pop().unwrap() {
                Token::Add => {
                    result += self.factor(&mut expr);
                }
                Token::Multiply => {
                    result *= self.factor(&mut expr);
                }
                Token::RightPrentices => {
                    return result;
                }
                _ => {
                    unreachable!();
                }
            }
        }
        result
    }

    fn factor(&self, mut expr: &mut Vec<&Token>) -> usize {
        match expr.pop().unwrap() {
            Token::Number(number) => *number,
            Token::LeftPrentices => self.evaluate(&mut expr),
            _ => {
                unreachable!();
            }
        }
    }
}

struct SolverPartTwo;
impl Solver for SolverPartTwo {
    fn evaluate(&self, mut expr: &mut Vec<&Token>) -> usize {
        let mut result = self.term(&mut expr);
        while !expr.is_empty() {
            match expr.pop().unwrap() {
                Token::Multiply => {
                    result *= self.term(&mut expr);
                }
                Token::RightPrentices => {
                    return result;
                }
                _ => {
                    unreachable!();
                }
            }
        }
        result
    }

    fn term(&self, mut expr: &mut Vec<&Token>) -> usize {
        let mut result = self.factor(&mut expr);
        while !expr.is_empty() {
            match expr.pop().unwrap() {
                Token::Add => {
                    result += self.factor(&mut expr);
                }
                other => {
                    expr.push(other);
                    return result;
                }
            }
        }
        result
    }

    fn factor(&self, mut expr: &mut Vec<&Token>) -> usize {
        match expr.pop().unwrap() {
            Token::Number(number) => *number,
            Token::LeftPrentices => self.evaluate(&mut expr),
            _ => {
                unreachable!();
            }
        }
    }
}

fn evaluate(solver: impl Solver, input: &str) -> Option<usize> {
    let mut expressions = parse_input_day18(input);
    expressions
        .iter_mut()
        .map(|expr| {
            let mut expr = expr.iter().rev().collect::<Vec<_>>();
            solver.evaluate(&mut expr)
        })
        .sum::<usize>()
        .into()
}

#[aoc(day18, part1)]
fn day18_part1(input: &str) -> Option<usize> {
    evaluate(SolverPartOne {}, input)
}

#[aoc(day18, part2)]
fn day18_part2(input: &str) -> Option<usize> {
    evaluate(SolverPartTwo {}, input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "1 + 2 * 3 + 4 * 5 + 6";
        assert_eq!(day18_part1(input), Some(71));

        let input = "2 * 3 + (4 * 5)";
        assert_eq!(day18_part1(input), Some(26));

        let input = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
        assert_eq!(day18_part1(input), Some(437));

        let input = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
        assert_eq!(day18_part1(input), Some(12240));

        let input = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        assert_eq!(day18_part1(input), Some(13632));
    }

    #[test]
    fn test_part2() {
        let input = "1 + 2 * 3 + 4 * 5 + 6";
        assert_eq!(day18_part2(input), Some(231));

        let input = "2 * 3 + (4 * 5)";
        assert_eq!(day18_part2(input), Some(46));

        let input = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
        assert_eq!(day18_part2(input), Some(1445));

        let input = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
        assert_eq!(day18_part2(input), Some(669060));

        let input = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        assert_eq!(day18_part2(input), Some(23340));
    }
}
