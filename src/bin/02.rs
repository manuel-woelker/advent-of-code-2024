advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut safe = 0u32;
    for line in input.lines() {
        let levels = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
        let all_ascending = levels.windows(2).all(|x| x[0] < x[1]);
        let all_descending = levels.windows(2).all(|x| x[0] > x[1]);
        let small_gaps = levels.windows(2).all(|x| (x[0]-x[1]).abs() <= 3);
        if (all_ascending || all_descending) && small_gaps {
            safe += 1;
        }

    }
    Some(safe)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut safe = 0u32;
    for line in input.lines() {
        let levels = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
        for i in 0..levels.len() {
            let mut levels = levels.clone();
            levels.remove(i);
            let all_ascending = levels.windows(2).all(|x| x[0] < x[1]);
            let all_descending = levels.windows(2).all(|x| x[0] > x[1]);
            let small_gaps = levels.windows(2).all(|x| (x[0]-x[1]).abs() <= 3);
            if (all_ascending || all_descending) && small_gaps {
                safe += 1;
                break;
            }
        }
    }
    Some(safe)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
