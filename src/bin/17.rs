advent_of_code::solution!(17);

type Int = i64;
const REG_A: usize = 0;
const REG_B: usize = 1;
const REG_C: usize = 2;

#[derive(Debug, Clone)]
struct IntCodeVm {
    pub registers: [i64;3],
    pub ip: Int,
    program: Vec<Int>,
    output: Vec<Int>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct State {
    pub registers: [i64;3],
    pub ip: Int,
}

impl IntCodeVm {
    pub fn new(registers: [Int;3], program: Vec<Int>) -> IntCodeVm {
        IntCodeVm {
            registers,
            ip: 0,
            program,
            output: vec![],
        }
    }

    pub fn parse(input: &str) -> IntCodeVm {
        let mut s = unscanny::Scanner::new(input);
        s.expect("Register A: ");
        let a = s.eat_while(char::is_ascii_digit).parse::<Int>().unwrap();
        s.expect("\nRegister B: ");
        let b = s.eat_while(char::is_ascii_digit).parse::<Int>().unwrap();
        s.expect("\nRegister C: ");
        let c = s.eat_while(char::is_ascii_digit).parse::<Int>().unwrap();
        s.expect("\n\nProgram: ");
        let mut program = vec![];
        loop {
            let operand = s.eat_while(char::is_ascii_digit).parse::<Int>().unwrap();
            program.push(operand);
            if !s.eat_if(",") {
                break;
            }
        }
        s.eat_whitespace();
        assert!(s.done());
        IntCodeVm::new([a, b, c], program)
    }

    pub fn run(&mut self) {
        loop {
            if self.ip >= self.program.len() as Int {
                break;
            }
            self.run_step();
        }
    }

    pub fn run_step(&mut self) {
        let instruction = self.program[self.ip as usize];
        match instruction {
            0 => { // ADV: Divide A by 2^operand
                let operand = self.get_combo_operand();
                self.registers[REG_A] = self.registers[REG_A] / 2i64.pow(operand as u32);
            }
            1 => { // BXL: Bitwise XOR B with literal operand
                let operand = self.get_literal_operand();
                self.registers[REG_B] = self.registers[REG_B] ^ operand;
            }
            2 => { // BST: B = operand modulo 8
                let operand = self.get_combo_operand();
                self.registers[REG_B] = operand % 8;
            }
            3 => { // JNZ: IF A != 0, jump to literal operand
                if self.registers[REG_A] != 0 {
                    self.ip = self.get_literal_operand(); return;
                }
            }
            4 => { // BXC: Bitwise XOR B and C
                self.registers[REG_B] = self.registers[REG_B] ^ self.registers[REG_C];
            }
            5 => { // OUT: Output combo operand modulo 8
                let operand = self.get_combo_operand();
                self.output.push(operand % 8);
            }
            6 => { // BDV: Divide A by 2^operand and store in B
                let operand = self.get_combo_operand();
                self.registers[REG_B] = self.registers[REG_A] / 2i64.pow(operand as u32);
            }
            7 => { // CDV: Divide A by 2^operand and store in C
                let operand = self.get_combo_operand();
                self.registers[REG_C] = self.registers[REG_A] / 2i64.pow(operand as u32);
            }
            _ => panic!("Invalid opcode at IP {}: {}", self.ip, instruction),
        }
        self.ip += 2;
    }

    fn get_combo_operand(&self) -> Int {
        let raw_operand = self.program[self.ip as usize+1];
        match raw_operand {
            0..=3 => raw_operand,
            4 => self.registers[REG_A],
            5 => self.registers[REG_B],
            6 => self.registers[REG_C],
            _ => panic!("Invalid operand at IP {}: {}", self.ip, raw_operand),
        }
    }

    fn get_literal_operand(&self) -> Int {
        self.program[self.ip as usize+1]
    }

    pub fn state(&self) -> State {
        State {
            registers: self.registers,
            ip: self.ip,
        }
    }
}
pub fn part_one(input: &str) -> Option<String> {
    let mut vm = IntCodeVm::parse(input);
    vm.run();
    Some(vm.output.iter().map(|v| format!("{}", v)).collect::<Vec<String>>().join(","))
}

pub fn part_two(input: &str) -> Option<u64> {
    let input_vm = IntCodeVm::parse(input);
    let mut input = 0;
//    'outer: for l in 1..1 {
    let mut last_result = 0;
    'outer: for l in 1..=input_vm.program.len() {
        let to_find = &input_vm.program[(input_vm.program.len()-l)..];
//        dbg!(to_find);
//        let end = input + 1000_000_000;
        let end = input + 10_000_000;
        for i in input..end {
            let max_steps = 1_000_000;
            let mut vm = input_vm.clone();
            //        let mut states = HashSet::new();

            vm.registers[REG_A] = i;
            //        vm.registers[REG_A] = 3*8+3*64+3*87*64;
            let mut steps = 0;
            loop {
                if vm.output.len() > vm.program.len() {
                    continue 'outer;
                }
                steps += 1;
                if steps > max_steps {
                    // Took too long
                    panic!("Exceeded max_steps {}", max_steps);
                }
                /*            if !states.insert(vm.state()) {
                                // Detected infinite loop
                                continue 'outer;
                            }*/
                if vm.ip >= vm.program.len() as Int {
                    break;
                }
                vm.run_step();
                //            dbg!(i);
                //            dbg!(vm.registers);
            }
//            println!("{} [{} {} {}] => {:?}", i, i % 8, i /  8 % 8, i % 64 / 8,vm.output);
            //if vm.output == vm.program {
            if !vm.output.is_empty() && vm.output.ends_with(to_find) {
//            if !vm.output.is_empty() && vm.output == to_find {
                println!("{} => {:?}", i, vm.output);
                last_result = i;
                input = (i % 8i64.pow(to_find.len() as u32))*8;
                //dbg!(input);
                continue 'outer;
            }
        }
        panic!("No valid input found for {} {:?}", input, to_find)
    }
    Some(last_result as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_one_3() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 3));
        assert_eq!(result, Some("0,3,5,4,3,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(117440));
    }

    fn test_program(initial_registers: [Int;3], program: Vec<Int>, expected_registers: [Int;3], expected_output: Vec<Int>) {
        let mut vm = IntCodeVm::new(initial_registers, program);
        vm.run();
        assert_eq!(vm.registers, expected_registers);
        assert_eq!(vm.output, expected_output);
    }

    macro_rules! test_programs {
        ($($name:ident: $input_registers:expr, $input_program:expr => $expected_registers:expr, $expect_output:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    test_program($input_registers, $input_program, $expected_registers, $expect_output);
                }
            )*
        };
    }

    test_programs!(
        test_empty: [0,0,0], vec![] => [0,0,0], vec![],

        test_adv_1: [16,0,9], vec![0,0] => [16,0,9], vec![],
        test_adv_2: [16,0,9], vec![0,1] => [8,0,9], vec![],
        test_adv_3: [16,0,9], vec![0,2] => [4,0,9], vec![],
        test_adv_4: [15,0,9], vec![0,2] => [3,0,9], vec![],
        test_adv_5: [16,0,9], vec![0,3] => [2,0,9], vec![],
        test_adv_6: [8,0,9], vec![0,3] => [1,0,9], vec![],
        test_adv_7: [7,0,9], vec![0,3] => [0,0,9], vec![],

        test_bxl_1: [1,7,3], vec![1,0] => [1,7,3], vec![],
        test_bxl_2: [1,7,3], vec![1,7] => [1,0,3], vec![],
        test_bxl_3: [1,3,3], vec![1,12] => [1,15,3], vec![],

        test_bst_1: [3,4,9], vec![2,6] => [3,1,9], vec![],
        test_bst_2: [3,8,9], vec![2,5] => [3,0,9], vec![],

        test_jnz_1: [1,8,9], vec![3,4, 5,0, 5,1] => [1,8,9], vec![1],
        test_jnz_0: [0,8,9], vec![3,4, 5,0, 5,1] => [0,8,9], vec![0, 1],

        test_bxc_1: [1,7,0], vec![4,0] => [1,7,0], vec![],
        test_bxc_2: [1,7,7], vec![4,7] => [1,0,7], vec![],
        test_bxc_3: [1,7,3], vec![4,12] => [1,4,3], vec![],

        test_out_0: [11,22,33], vec![5,0] => [11,22,33], vec![0],
        test_out_1: [11,22,33], vec![5,1] => [11,22,33], vec![1],
        test_out_2: [11,22,33], vec![5,2] => [11,22,33], vec![2],
        test_out_3: [11,22,33], vec![5,3] => [11,22,33], vec![3],
        test_out_4: [11,22,33], vec![5,4] => [11,22,33], vec![3],
        test_out_5: [11,22,33], vec![5,5] => [11,22,33], vec![6],
        test_out_6: [11,22,33], vec![5,6] => [11,22,33], vec![1],

        test_bdv_1: [16,0,9], vec![6,0] => [16,16,9], vec![],
        test_bdv_2: [16,0,9], vec![6,1] => [16,8,9], vec![],
        test_bdv_3: [16,0,9], vec![6,2] => [16,4,9], vec![],
        test_bdv_4: [15,0,9], vec![6,2] => [15,3,9], vec![],
        test_bdv_5: [16,0,9], vec![6,3] => [16,2,9], vec![],
        test_bdv_6: [8,0,9], vec![6,3] => [8,1,9], vec![],
        test_bdv_7: [7,0,9], vec![6,3] => [7,0,9], vec![],

        test_cdv_1: [16,0,9], vec![7,0] => [16,0,16], vec![],
        test_cdv_2: [16,0,9], vec![7,1] => [16,0,8], vec![],
        test_cdv_3: [16,0,9], vec![7,2] => [16,0,4], vec![],
        test_cdv_4: [15,0,9], vec![7,2] => [15,0,3], vec![],
        test_cdv_5: [16,0,9], vec![7,3] => [16,0,2], vec![],
        test_cdv_6: [8,0,9], vec![7,3] => [8,0,1], vec![],
        test_cdv_7: [7,0,9], vec![7,3] => [7,0,0], vec![],

        test_ex_0: [11,22,33], vec![] => [11,22,33], vec![],

        test_ex_1: [11,22,9], vec![2,6] => [11,1,9], vec![],
        test_ex_2: [10,22,33], vec![5,0,5,1,5,4] => [10,22,33], vec![0,1,2],
        test_ex_3: [2024,22,33], vec![0,1,5,4,3,0] => [0,22,33], vec![4,2,5,6,7,7,7,7,3,1,0],
        test_ex_4: [11,29,33], vec![1,7] => [11,26,33], vec![],
        test_ex_5: [11,2024,43690], vec![4,0] => [11,44354,43690], vec![],


    );
/*
    #[test]
    fn test_empty() {
        let mut vm = IntCodeVm::new([0,0,0], vec![]);
        vm.run();
        assert_eq!(vm.registers, [0,0,0]);
        assert_eq!(vm.output, []);
    }

    #[test]
    fn test_bst() {
        let mut vm = IntCodeVm::new([3,4,9], vec![2,6]);
        vm.run();
        assert_eq!(vm.registers, [3,1,9]);
        assert_eq!(vm.output, []);
    }*/
}
