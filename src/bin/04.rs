advent_of_code::solution!(4);


pub fn part_one(input: &str) -> Option<u32> {
    let needles = ["XMAS".chars().collect::<Vec<char>>(), "SAMX".chars().collect::<Vec<char>>()];
    let data: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut count = 0;
    let width = data[0].len();
    let height = data.len();
    for needle in needles.iter() {
        for line in &data {
            // count occurrences of needles in lines
            count += line.windows(needle.len()).filter(|x| x == needle).count();
        }
        // Straight down
        for x in 0..width {
            for y in 0..=height-needle.len() {
                let mut candidate = vec![];
                for i in 0..needle.len() {
                    candidate.push(data[y+i][x]);
                }
                if candidate == *needle {
                    count += 1;
                }
            }
        }
        // Right down
        for x in 0..=width-needle.len() {
            for y in 0..=height-needle.len() {
                let mut candidate = vec![];
                for i in 0..needle.len() {
                    candidate.push(data[y+i][x+i]);
                }
                if candidate == *needle {
                    count += 1;
                }
            }
        }
        // Left down
        for x in needle.len()-1..width {
            for y in 0..=height-needle.len() {
                let mut candidate = vec![];
                for i in 0..needle.len() {
                    candidate.push(data[y+i][x-i]);
                }
                if candidate == *needle {
                    count += 1;
                }
            }
        }
    }

    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut count = 0;
    let width = data[0].len();
    let height = data.len();
    for x in 0..width-2 {
        for y in 0..height-2 {
            if data[y+1][x+1] == 'A' {
                if (data[y][x] == 'M' && data[y+2][x+2] == 'S') || (data[y][x] == 'S' && data[y+2][x+2] == 'M') {
                    if (data[y][x+2] == 'M' && data[y+2][x] == 'S') || (data[y][x+2] == 'S' && data[y+2][x] == 'M') {
                        count += 1;
                    }
                }
            }
        }
    }

    Some(count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
