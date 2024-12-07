use unscanny::Scanner;

advent_of_code::solution!(7);

#[derive(Debug)]
struct Equation {
    result: u64,
    operands: Vec<u64>,
}

pub enum Operator {
    Add,
    Multiply,
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut calibration_result = 0;

    for line in input.lines() {
        let mut s = Scanner::new(line);
        let result = s.eat_while(char::is_ascii_digit).parse::<u64>().unwrap();
        s.expect(":");
        let mut operands = vec![];
        while !s.done() {
            s.expect(" ");
            let operand = s.eat_while(char::is_ascii_digit).parse::<u64>().unwrap();
            operands.push(operand);
        }
        if can_compute_result(operands[0], &operands[1..], result) {
           calibration_result += result;
        }
    }


    Some(calibration_result)
}

fn can_compute_result(accum: u64, operands: &[u64], result: u64) -> bool {
    if operands.is_empty() {
        let can_compute = accum == result;
        return can_compute;
    }
    let first_operand = operands[0];
    can_compute_result(accum+first_operand, &operands[1..], result) || can_compute_result(accum*first_operand, &operands[1..], result)
}

fn can_compute_result2(accum: u64, operands: &[u64], result: u64) -> bool {
    if operands.is_empty() {
        let can_compute = accum == result;
        return can_compute;
    }
    let first_operand = operands[0];
    can_compute_result2(accum+first_operand, &operands[1..], result) || can_compute_result2(accum*first_operand, &operands[1..], result)  || can_compute_result2(concat(accum, first_operand), &operands[1..], result)
}

fn concat(a: u64, b: u64) -> u64 {
    let mut x = b;
    let mut result = a;
    while x > 0 {
        x /= 10;
        result *= 10;
    }
    result+b
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut calibration_result = 0;

    for line in input.lines() {
        let mut s = Scanner::new(line);
        let result = s.eat_while(char::is_ascii_digit).parse::<u64>().unwrap();
        s.expect(":");
        let mut operands = vec![];
        while !s.done() {
            s.expect(" ");
            let operand = s.eat_while(char::is_ascii_digit).parse::<u64>().unwrap();
            operands.push(operand);
        }
        if can_compute_result2(operands[0], &operands[1..], result) {
            calibration_result += result;
        }
    }


    Some(calibration_result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
