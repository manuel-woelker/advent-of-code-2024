advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = input.lines().map(|line| line.split_once("   ").unwrap()).map(|(a,b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap())).unzip();
    left.sort();
    right.sort();
    let sum: i32 = left.iter().zip(right).map(|(a, b)| (a - b).abs()).sum();
    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = input.lines().map(|line| line.split_once("   ").unwrap()).map(|(a,b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap())).unzip();
    left.sort();
    right.sort();
    let mut similarity = 0;
    for i in left {
        let count = right.iter().filter(|candidate| *candidate == &i).count();
        similarity += count as i32*i;
    }
    Some(similarity as u32)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
