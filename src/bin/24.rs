use std::collections::{HashMap, HashSet};
use std::mem::swap;
use unscanny::Scanner;

advent_of_code::solution!(24);

#[derive(Debug, PartialEq, Eq, Hash)]
enum Op {
    And,
    Or,
    Xor,
}

#[derive(Debug)]
struct Gate {
    op: Op,
    a: String,
    b: String,
    output: String,
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut scanner = Scanner::new(input);
    let mut values = HashMap::<String, bool>::new();
    let mut gates = Vec::new();
    let mut wire_map:HashMap<String, Vec<&Gate>> = HashMap::new();
    while !scanner.eat_if("\n") {
        let name = scanner.eat_while(char::is_alphanumeric).to_string();
        scanner.expect(": ");
        let value = scanner.eat();
        values.insert(name, value == Some('1'));
        scanner.expect("\n");
    }
    while !scanner.done() {
        let a = scanner.eat_while(char::is_alphanumeric).to_string();
        scanner.expect(" ");
        let op_string = scanner.eat_while(char::is_alphabetic);
        scanner.expect(" ");
        let b = scanner.eat_while(char::is_alphanumeric).to_string();
        scanner.expect(" -> ");
        let output = scanner.eat_while(char::is_alphanumeric).to_string();
        let op = match op_string {
            "AND" => Op::And,
            "OR" => Op::Or,
            "XOR" => Op::Xor,
            _ => unreachable!(),
        };
        gates.push(Gate { op, a, b, output });
        scanner.expect("\n");
    }
    for gate in &gates {
        wire_map.entry(gate.a.clone()).or_default().push(gate);
        wire_map.entry(gate.b.clone()).or_default().push(gate);
    }
    let mut todos: HashSet<String> = values.keys().cloned().collect();
    while let Some(todo) = todos.iter().cloned().next() {
        todos.remove(&todo);

        let gates = wire_map.entry(todo).or_default();
        for gate in gates {
            if values.contains_key(&gate.output) {
                continue;
            }
            let value_a = values.get(&gate.a);
            let value_b = values.get(&gate.b);
            if let (Some(a), Some(b)) = (value_a, value_b) {
                let value = match gate.op {
                    Op::And => *a && *b,
                    Op::Or => *a || *b,
                    Op::Xor => *a ^ *b,
                };
                values.insert(gate.output.clone(), value);
                todos.insert(gate.output.clone());
            }
        }
    }
    let mut result = 0;
    for i in 0..100 {
        let wire = format!("z{:02}", i);
//        dbg!(&wire);
        let Some(value) = values.get(&wire) else {
            break;
        };
//        println!("{}: {}", wire, value);
        result += if *value { 1 << i } else { 0 };
    }

//    dbg!(result);
    Some(result)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut scanner = Scanner::new(input);
    let mut gates = Vec::new();
    let mut wire_map: HashMap<String, Vec<&Gate>> = HashMap::new();
    let mut wire_map2: HashMap<String, &Gate> = HashMap::new();
    let mut output_map: HashMap<String, &Gate> = HashMap::new();
    // Logical name to gate
    let logical_map: HashMap<String, &Gate> = HashMap::new();
    while !scanner.eat_if("\n") {
        let name = scanner.eat_while(char::is_alphanumeric).to_string();
        scanner.expect(": ");
        let value = scanner.eat();
        scanner.expect("\n");
    }
    while !scanner.done() {
        let a = scanner.eat_while(char::is_alphanumeric).to_string();
        scanner.expect(" ");
        let op_string = scanner.eat_while(char::is_alphabetic);
        scanner.expect(" ");
        let b = scanner.eat_while(char::is_alphanumeric).to_string();
        scanner.expect(" -> ");
        let output = scanner.eat_while(char::is_alphanumeric).to_string();
        let op = match op_string {
            "AND" => Op::And,
            "OR" => Op::Or,
            "XOR" => Op::Xor,
            _ => unreachable!(),
        };
        gates.push(Gate { op, a, b, output });
        scanner.expect("\n");
    }
    let mut wires = gates.iter().map(|g| g.output.clone()).collect::<Vec<_>>();
    wires.sort();
    dbg!(wires.last());
    for gate in &gates {
        wire_map.entry(gate.a.clone()).or_default().push(gate);
        wire_map.entry(gate.b.clone()).or_default().push(gate);
        let mut a = &gate.a;
        let mut b = &gate.b;
        if a > b {
            swap(&mut a, &mut b);
        }
        assert!(wire_map2.insert(format!("{} {:?} {}", a, gate.op, b), gate).is_none());
        output_map.insert(gate.output.clone(), gate);
    }
    let number_of_input_bits = 44;
    //dbg!(&wire_map2);
    // x00 AND y00 -> z00
    for i in 0..=number_of_input_bits {
        let key = format!("x{:02} {:?} y{:02}", i, Op::Xor, i);
//        dbg!(&key);
        let gate = wire_map2.get(&key).unwrap();
        //dbg!(&gate.output);
        wires.push(gate.output.clone());
    }
    let mut i = number_of_input_bits;
    let mut wrong_outputs = HashSet::new();
/*    loop {
        if i < 2 {
            break;
        }
        i-= 1;
        println!("i: {}", i);
        let in_a = format!("x{:02}", i);
        let in_b = format!("y{:02}", i);
        let last_carry_gate = output_map.get(&format!("z{:02}",i)).unwrap();
        if last_carry_gate.op != Op::Xor {
            println!("FOUND MISMATCHED OUTPUT");
            wrong_outputs.insert(last_carry_gate.output.clone());
            dbg!(last_carry_gate);
            // Try to find correct gate output
            let and_gate = wire_map2.get(&format!("{} {:?} {}", in_a, Op::And, in_b)).unwrap();
            dbg!(and_gate);
            let output = &and_gate.output;
            dbg!(output);
            for gate in &gates {
                if /*gate.op == Op::Xor &&*/ (&gate.a == output || &gate.b == output) {
                    dbg!(gate);
                }
            }
            continue;
        }
        let carry_a = output_map.get(&last_carry_gate.a).unwrap();
        let carry_b = output_map.get(&last_carry_gate.b).unwrap();
        let other_carry_gate;
        if (carry_a.a == in_a && carry_a.b == in_b) || (carry_a.a == in_b && carry_a.b == in_a) {
            other_carry_gate = carry_b;
        } else if (carry_b.a == in_a && carry_b.b == in_b) || (carry_b.a == in_b && carry_b.b == in_a){
            other_carry_gate = carry_a;
        } else {
            panic!("Could not find carry gate for {} and {}", in_a, in_b);
        }
//        dbg!(carry_a);
//        dbg!(carry_b);
    }

 */
/*    for gate in &gates {
        if !(gate.a.starts_with("x") || gate.a.starts_with("y") || gate.b.starts_with("x") || gate.b.starts_with("y") || gate.output.starts_with("z")) {
            if gate.op == Op::Xor {
                dbg!(gate);
                wrong_outputs.insert(gate.output.clone());
            }
        }
    }
    let last_carry_gate = output_map.get("z44").unwrap();
    dbg!(last_carry_gate);
    let carry_a = output_map.get(&last_carry_gate.a).unwrap();
    let carry_b = output_map.get(&last_carry_gate.b).unwrap();
    dbg!(carry_a);
    dbg!(carry_b);*/

    for gate in &gates {
        match gate.op {
            Op::Xor => {
                if !(gate.a.starts_with("x") || gate.a.starts_with("y") || gate.b.starts_with("x") || gate.b.starts_with("y")) {
                    if !gate.output.starts_with("z") {
                        wrong_outputs.insert(gate.output.clone());
                    }
                } else {
                    if !(gate.a.starts_with("x") || gate.a.starts_with("y")) && (gate.b.starts_with("x") || gate.b.starts_with("y")) {
                        println!("UNEXPECTED");
                    }
                    if gate.output == "z00" {
                        continue;
                    }
                    if gate.output.starts_with("z") {
                        wrong_outputs.insert(gate.output.clone());
                        continue;
                    }
                    let Some(next_gates) = wire_map.get(&gate.output) else {
                        dbg!(&gate.output);
                        wrong_outputs.insert(gate.output.clone());
                        continue;
                    };
                    let mut has_and = false;
                    let mut has_xor = false;
                    for next_gate in next_gates {
                        if next_gate.op == Op::And {
                            has_and = true;
                        } else if next_gate.op == Op::Xor {
                            has_xor = true;
                        }
                    }
                    if !(has_and || has_xor) {
                        wrong_outputs.insert(gate.output.clone());
                    }
                }
            }
            Op::Or => {
                if gate.output.starts_with("z") {
                    if gate.output == "z45" {
                        continue;
                    }
                    wrong_outputs.insert(gate.output.clone());
                    continue;
                }
                let Some(next_gates) = wire_map.get(&gate.output) else {
                    wrong_outputs.insert(gate.output.clone());
                    continue;
                };
                let mut has_and = false;
                let mut has_xor = false;
                for next_gate in next_gates {
                    if next_gate.op == Op::And {
                        has_and = true;
                    } else if next_gate.op == Op::Xor {
                        has_xor = true;
                    }
                }
                if !(has_and && has_xor) {
                    wrong_outputs.insert(gate.output.clone());
                }
            }
            Op::And => {
                if gate.output.starts_with("z")||gate.output.starts_with("x")||gate.output.starts_with("y") {
                    wrong_outputs.insert(gate.output.clone());
                    continue;
                }
                let Some(next_gates) = wire_map.get(&gate.output) else {
                    wrong_outputs.insert(gate.output.clone());
                    continue;
                };
                let mut has_or = false;
                for next_gate in next_gates {
                    if next_gate.op == Op::Or {
                        has_or = true;
                    }
                }
                if !(has_or) {
                    wrong_outputs.insert(gate.output.clone());
                }
            }
        }
    }
    let mut wrong_output_list: Vec<_> = wrong_outputs.into_iter().collect();
    wrong_output_list.sort();
    dbg!(&wrong_output_list);
    dbg!(&wrong_output_list.len());
    Some(wrong_output_list.join(","))
    // WRONG: drg,gvw,jbp,jgc,nvv,z00,z15,z22,z35
    // WRONG: drg,gvw,jbp,jgc,nvv,qjb,z15,z22,z35

    // CORRECT: drg,gvw,jbp,jgc,qjb,z15,z22,z35
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY,2));
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
