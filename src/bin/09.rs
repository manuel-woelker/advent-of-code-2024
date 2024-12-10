advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let compact_map: Vec<u32> = input.chars().map(|x| x.to_digit(10).unwrap()).collect();
    let mut map = Vec::new();
    let mut is_block = true;
    let mut file_id = -1i64;
    for entry in compact_map {
        let value = if is_block {
            file_id += 1;
            file_id
        } else {
            -1
        };
        for _ in 0..entry {
            map.push(value);
        }
        is_block = !is_block;
    }

    let mut write_pos = 0;
    while write_pos < map.len() {
        // find free spot
        if map[write_pos] != -1 {
            write_pos += 1;
            continue;
        }
        // find value to insert
        let value = loop {
            let val = map.pop().unwrap();
            if val != -1 {
                break val;
            }
        };
        // insert value
        map[write_pos] = value;
    }
/*
    for i in map {
        if i == -1 {
            print!(".");
        } else {
            print!("{}", i);
        }
    }
    println!();
*/
    let mut sum = 0u64;
    for (pos, val) in map.iter().enumerate() {
        sum += (pos as i64 * val) as u64;
    }

    Some(sum)
}

struct Entry {
    width: usize,
    value: i64,
}

pub fn part_two(input: &str) -> Option<u64> {
    let compact_map: Vec<u32> = input.chars().map(|x| x.to_digit(10).unwrap()).collect();
    let mut map = Vec::new();
    let mut is_block = true;
    let mut file_id = -1i64;
    for entry in compact_map {
        let value = if is_block {
            file_id += 1;
            file_id
        } else {
            -1
        };
        map.push(Entry { width: entry as usize, value });
        is_block = !is_block;
    }

    let mut read_pos = map.len();
    while read_pos > 0 {
        read_pos -= 1;
        let entry = &map[read_pos];
        if entry.value == -1 {
            continue;
        }
        // find free spot
        let mut found = -1i64;
        let width = map[read_pos].width;
        for i in 0..read_pos {
            let hole = &map[i];
            if hole.value == -1 && hole.width >= width {
                found = i as i64;
                break;
            }
        }
        if found == -1 {
            continue;
        }
        let value = map[read_pos].value;
        let width = map[read_pos].width;
//        println!("{value} from {read_pos} -> {found} ({width})");
        map[found as usize].width -= width;
        map[read_pos].value = -1;
        map.insert(found as usize, Entry { width, value });
    }

    let mut sum = 0u64;
    let mut pos = 0u64;
    for entry in &map {
        let value = if entry.value == -1 {
            "."
        } else { &entry.value.to_string() };
        if entry.value != -1 {
            for _ in 0..entry.width {
                //            print!("{}", value);
                sum += pos * entry.value as u64;
                pos += 1;
            }
        } else {
            pos += entry.width as u64;
        }
    }
    println!();
    /*
        for i in map {
            if i == -1 {
                print!(".");
            } else {
                print!("{}", i);
            }
        }
        println!();
    */
/*    for (pos, val) in map.iter().enumerate() {
        sum += (pos as i64 * val) as u64;
    }*/

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
