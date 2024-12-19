advent_of_code::solution!(19);



pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let first_line = input.lines().next().unwrap();
    let available_towels: Vec<&str> = first_line.split(", ").collect();
    lines.next().unwrap();
    let mut possible_patterns = 0;
    'outer: for line in lines {
        let mut todo = vec![line];
        while let Some(rest) = todo.pop() {
            for available_towel in &available_towels {
                if rest.starts_with(available_towel) {
                    let next = &rest[available_towel.len()..];
                    if next.is_empty() {
                        possible_patterns += 1;
                        continue 'outer;
                    }
                    todo.push(next);
                }
            }
        }
    }
    Some(possible_patterns)
}

pub fn part_two_brute(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let first_line = input.lines().next().unwrap();
    let available_towels: Vec<&str> = first_line.split(", ").collect();
    lines.next().unwrap();
    let mut possible_patterns = 0;
    for line in lines {
        let mut todo = vec![line];
        while let Some(rest) = todo.pop() {
            for available_towel in &available_towels {
                if rest.starts_with(available_towel) {
                    let next = &rest[available_towel.len()..];
                    if next.is_empty() {
                        possible_patterns += 1;
                    }
                    todo.push(next);
                }
            }
        }
    }
    Some(possible_patterns)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut lines = input.lines();
    let first_line = input.lines().next().unwrap();
    let available_towels: Vec<&str> = first_line.split(", ").collect();
    lines.next().unwrap();
    lines.next().unwrap();
    let mut possible_patterns = 0;
    for line in lines {
        let mut possible_patterns_for_length = vec![0usize; line.len() + 100];
        possible_patterns_for_length[0] = 1;

        for i in 0..line.len() {
            let to_match = &line[i..];
            let patterns_to_here = possible_patterns_for_length[i];
            for available_towel in &available_towels {
                if to_match.starts_with(available_towel) {
                    possible_patterns_for_length[i+available_towel.len()] += patterns_to_here;
                }
            }
        }
        println!("{} => {}", line, possible_patterns_for_length[line.len()]);
        //dbg!(possible_patterns_for_length[line.len()]);
        possible_patterns += possible_patterns_for_length[line.len()];
/*        let mut todo = vec![line];
        while let Some(rest) = todo.pop() {
            for available_towel in &available_towels {
                if rest.starts_with(available_towel) {
                    let next = &rest[available_towel.len()..];
                    if next.is_empty() {
                        possible_patterns += 1;
                    }
                    todo.push(next);
                }
            }
        }*/
//        dbg!(possible_patterns);
    }
    Some(possible_patterns)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
