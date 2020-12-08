use itertools::Itertools;
use aoc_runner_derive::aoc;

#[derive(Debug, Clone)]
enum OpCode {
    NOP,
    JMP,
    ACC,
}

impl OpCode {
    fn from(opcode_string: &str) -> Self {
        match opcode_string {
            "nop" => OpCode::NOP,
            "jmp" => OpCode::JMP,
            "acc" => OpCode::ACC,
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    opcode: OpCode,
    value: isize,
}

impl Instruction {
    fn new(opcode: &str, value: isize) -> Self {
        Self {
            opcode: OpCode::from(opcode),
            value
        }
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            line
                .splitn(2, " ")
                .collect_tuple()
                .unwrap()
        })
        .map(|(opcode, value)| {
            Instruction::new(&opcode, value.parse::<isize>().unwrap())
        })
        .collect::<Vec<_>>()
}

fn execute(instructions: &[Instruction]) -> Option<(usize, usize)> {
    let mut accumulator = 0;
    let mut pc: isize = 0;
    let mut visits = vec![false; instructions.len()];
    loop {
        if let Some(instruction) = instructions.get(pc as usize) {
            if visits[pc as usize] {
                break;
            }
            let mut pc_diff = 1;
            match instruction.opcode {
                OpCode::ACC => accumulator += instruction.value,
                OpCode::JMP => pc_diff = instruction.value,
                OpCode::NOP => {},
            };
            visits[pc as usize] = true;
            pc += pc_diff;
        } else {
            break;
        }
    }
    Some((pc as usize, accumulator as usize))
}

#[aoc(day8, part1)]
fn day8_part1(input: &str) -> Option<usize> {
    let mut instructions = parse(input);
    Some(execute(&mut instructions).unwrap().1)
}

#[aoc(day8, part2)]
fn day8_part2(input: &str) -> Option<usize> {
    let instructions = parse(input);

    for (i, instruction) in instructions.iter().enumerate() {
        let mut copy = parse(input); // STUPID Step!!
        copy.get_mut(i).unwrap().opcode = match instruction.opcode {
            OpCode::JMP => OpCode::NOP,
            OpCode::NOP => OpCode::JMP,
            _ => continue
        };
        if let Some((last_pc, value)) = execute(&copy) {
            if last_pc >= instructions.len() {
                return Some(value);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = 
"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        assert_eq!(day8_part1(input), Some(5));
    }

    #[test]
    fn test_part2() {
        let input = 
"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        assert_eq!(day8_part2(input), Some(8));
    }
}
