use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut result = 0;
    for capture in re.captures_iter(input) {
        let a: u32 = capture.get(1).unwrap().as_str().parse().unwrap();
        let b: u32 = capture.get(2).unwrap().as_str().parse().unwrap();
        result += a*b;
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"(do\(\))|(don't\(\))|(mul\((\d+),(\d+)\))").unwrap();
    let mut enabled = true;
    let mut result = 0;
    for capture in re.captures_iter(input) {
        if capture.get(1).is_some() {
            enabled = true;
        } else if capture.get(2).is_some() {
            enabled = false;
        } else if capture.get(3).is_some() {
            if enabled {
                let a: u32 = capture.get(4).unwrap().as_str().parse().unwrap();
                let b: u32 = capture.get(5).unwrap().as_str().parse().unwrap();
                result += a*b;
            }
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
