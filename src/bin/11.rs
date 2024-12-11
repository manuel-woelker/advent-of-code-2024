use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u32> {
    let mut number_of_stones = 0;
    let number_of_steps = 25;
    let mut todo: Vec<(i64, i64)> = vec![];
    for stone in input.split(" ") {
        todo.push((stone.parse().unwrap(), number_of_steps));
    }

    while let Some((mut stone, steps)) = todo.pop() {
        number_of_stones += 1;
        if steps == 0 {
            continue;
        }
  //      dbg!(stone);
        for step in (0..steps).rev() {
//            dbg!(stone);
            if stone == 0 {
                stone = 1;
                continue;
            }
            // get number of digits in stone
            let mut count = 0;
            let mut tmp = stone;
            while tmp > 0 {
                count += 1;
                tmp /= 10;
            }
            if count % 2 == 0 {
                let divisor = 10i64.pow(count /2);
                todo.push((stone % divisor, step));
                stone /= divisor;
            } else {
                stone *= 2024;
            }
        }
    }
    Some(number_of_stones)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut number_of_stones = 0;
    let number_of_steps = 75;
    let mut todo: Vec<(i64, i64)> = vec![];
    for stone in input.split(" ") {
        todo.push((stone.parse().unwrap(), number_of_steps));
    }
    let mut cache: HashMap <(i64, i64), u64> = HashMap::new();
    while let Some((stone, steps)) = todo.pop() {
        number_of_stones += compute_number_of_stones(stone, steps, &mut cache)
    }
    Some(number_of_stones)

}

fn compute_number_of_stones(stone: i64, steps: i64, cache: &mut HashMap<(i64, i64), u64>) -> u64 {
    if steps == 0 {
        return 1;
    }
    if let Some(number) = cache.get(&(stone, steps)) {
        return *number;
    }
    if stone == 0 {
        let number_of_stones = compute_number_of_stones(1, steps - 1, cache);
        cache.insert((stone, steps), number_of_stones);
        return number_of_stones;
    }
    // get number of digits in stone
    let mut count = 0;
    let mut tmp = stone;
    while tmp > 0 {
        count += 1;
        tmp /= 10;
    }
    let result = if count % 2 == 0 {
        let divisor = 10i64.pow(count /2);
        let a = compute_number_of_stones(stone % divisor, steps - 1, cache);
        let b = compute_number_of_stones(stone / divisor, steps - 1, cache);
        a + b
    } else {
        compute_number_of_stones(stone * 2024, steps - 1, cache)
    };
    cache.insert((stone, steps), result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }
}
