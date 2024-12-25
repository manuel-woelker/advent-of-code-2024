advent_of_code::solution!(25);


#[derive(Debug)]
struct Shape {
    heights: [u32; 5],
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let mut keys = Vec::<Shape>::new();
    let mut locks = Vec::<Shape>::new();
    loop {
        let Some(line) = lines.next() else {
            break;
        };
        if line == "#####" {
            let mut heights = [0u32;5];
            for i in 0..10 {
                let Some(line) = lines.next() else {
                    break;
                };
                if line == "" {
                    break;
                }
                for j in 0..5 {
                    if line.as_bytes()[j] == b'#' {
                        heights[j] += 1;
                    }
                }
            }
            locks.push(Shape {heights});
        } else if line == "....." {
            let mut heights = [0u32;5];
            for i in 0..10 {
                let Some(line) = lines.next() else {
                    break;
                };
                if line == "" {
                    break;
                }
                for j in 0..5 {
                    if line.as_bytes()[j] == b'#' {
                        heights[j] += 1;
                    }
                }
            }
            for height in &mut heights {
                *height -= 1;
            }
            keys.push(Shape {heights});

        } else {
            unreachable!("Invalid input");
        }
    }
    let mut fits = 0;
    for key in &keys {
        for lock in &locks {
            if key.heights.iter().zip(lock.heights.iter()).all(|(a,b)| a + b <= 5) {
                fits += 1;
            }
        }
    }
//    dbg!(&keys);
//    dbg!(&locks);
    Some(fits)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
