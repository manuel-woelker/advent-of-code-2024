use std::collections::{HashMap, HashSet};
use std::ops::AddAssign;

advent_of_code::solution!(22);

fn next_secret_number(current: u64) -> u64 {
    let number = mix(current * 64, current);
    let number = mix(number / 32, number);
    let number = mix(number * 2048, number);
    number
}

fn mix(a: u64, b: u64) -> u64 {
    prune(a ^ b)
}

fn prune (a: u64) -> u64 {
    a % 16777216
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum = 0;
    for line in input.lines() {
        let start = line.parse::<u64>().unwrap();
        let mut number = start;
        for _i in 0..2000 {
            number = next_secret_number(number);
        }
        sum += number;
//        println!("{} -> {}", start, number);
    }
    Some(sum)
}

type Index = [u8;4];

struct Entry {
    sum: u64,
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sums = HashMap::new();
    for line in input.lines() {
        let mut sold = HashSet::new();
        let mut changes = [99i8; 4];
        let start = line.parse::<u64>().unwrap();
        let mut number = start;
        let mut old_price = (start % 10) as i32;
        for _i in 0..2000 {
            number = next_secret_number(number);
            let new_price = (number % 10) as i32;
            let change = new_price - old_price;
//            println!("{:10} -> {} ({})", number, new_price, change);
            old_price = new_price;
            changes.rotate_left(1);
            changes[3] = change as i8;
            if sold.insert(changes) {
                sums.entry(changes).or_insert(0u64).add_assign(new_price as u64);
            }
//            dbg!(changes);
        }
    }
    //dbg!(sums);
    let max = sums.values().max().unwrap();
/*    for (k, v) in sums.iter() {
        if v == max {
            println!("{:?} -> {}", k, v);
        }
    }*/
    Some(*max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(23));
    }

//    #[test]
    fn test_part_two_x() {
        let result = part_two("123");
        assert_eq!(result, None);
    }

    #[test]
    fn test_next_secret_number() {
        assert_eq!(next_secret_number(123), 15887950);
        let sequence = [123, 15887950
        ,16495136
        ,527345
        ,704524
        ,1553684
        ,12683156
        ,11100544
        ,12249484
        ,7753432
        ,5908254];
        for (a, b) in sequence.iter().zip(sequence.iter().skip(1)) {
            assert_eq!(next_secret_number(*a), *b);
        }
    }

    #[test]
    fn test_mix() {
        assert_eq!(mix(42, 15), 37);
    }

    #[test]
    fn test_prune() {
        assert_eq!(prune(100000000), 16113920);
    }

}
