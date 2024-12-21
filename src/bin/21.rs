use std::collections::HashMap;
use advent_of_code::map::{Map, Scalar};

advent_of_code::solution!(21);

pub fn part_one(input: &str) -> Option<u64> {
    let mut map = HashMap::new();
    let codes = input.lines().collect::<Vec<_>>();
    let mut result = 0;
    for code in codes {
        //dbg!(code);
        let length = get_shortest_path_length_numeric(code, 2, &mut map);
        let ncode: i64 = code[0..3].parse().unwrap();
//        dbg!(ncode);
//        dbg!(length);
        result += length * ncode;
    }
    Some(result as u64)
}

fn get_shortest_path_length_numeric(code: &str, depth: u32, memo_map: &mut HashMap<(String, u32), i64>) -> i64 {
    let mut numeric_map = Map::new(3, 4);
    numeric_map[(0, 0)] = '7';
    numeric_map[(1, 0)] = '8';
    numeric_map[(2, 0)] = '9';
    numeric_map[(0, 1)] = '4';
    numeric_map[(1, 1)] = '5';
    numeric_map[(2, 1)] = '6';
    numeric_map[(0, 2)] = '1';
    numeric_map[(1, 2)] = '2';
    numeric_map[(2, 2)] = '3';
    numeric_map[(0, 3)] = 'X';
    numeric_map[(1, 3)] = '0';
    numeric_map[(2, 3)] = 'A';
    let mut shortest_paths = vec![];
    let code = "A".to_string()+code;
    for (first, second) in code.chars().zip(code.chars().skip(1)) {
        let start_pos = numeric_map.find_tile_pos(|tile| tile == &first).unwrap();
        let end_pos = numeric_map.find_tile_pos(|tile| tile == &second).unwrap();
        let dx = end_pos.0 - start_pos.0;
        let dy = end_pos.1 - start_pos.1;
        let xseg = match dx {
            0 => "",
            1 => ">",
            2 => ">>",
            -1 => "<",
            -2 => "<<",
            _ => unreachable!(),
        };
        let yseg = match dy {
            0 => "",
            1 => "v",
            2 => "vv",
            3 => "vvv",
            -1 => "^",
            -2 => "^^",
            -3 => "^^^",
            _ => unreachable!(),
        };
        if xseg == "" || yseg == "" {
            shortest_paths.push(vec![xseg.to_string()+yseg+"A"]);
        } else {
//            shortest_paths.push(vec![xseg.to_string()+yseg+"A", yseg.to_string()+xseg+"A"]);
            if start_pos.0 == 0 && end_pos.1 == 3 {
                shortest_paths.push(vec![xseg.to_string()+yseg+"A"]);
            } else if end_pos.0 == 0 && start_pos.1 == 3 {
                shortest_paths.push(vec![yseg.to_string()+xseg+"A"]);
            } else {
                shortest_paths.push(vec![xseg.to_string()+yseg+"A", yseg.to_string()+xseg+"A"]);
            }

        }
    }
//    dbg!(&shortest_paths);
    let mut length = 0;
    for path in shortest_paths {
        length += path.iter().map(|s| {
            get_shortest_path_keypad(s, depth - 1, memo_map)
        }).min().unwrap();
    }
    length as i64
}

fn get_shortest_path_keypad(original_code: &str, depth: u32, memo_map: &mut HashMap<(String, u32), i64>) -> i64 {
//    dbg!(&memo_map, original_code, depth);
    if let Some(result) = memo_map.get(&(original_code.to_string(), depth)) {
//        dbg!(result);
        return *result;
    }
    let mut keypad_map = Map::new(3, 2);
    keypad_map[(0, 0)] = 'X';
    keypad_map[(1, 0)] = '^';
    keypad_map[(2, 0)] = 'A';
    keypad_map[(0, 1)] = '<';
    keypad_map[(1, 1)] = 'v';
    keypad_map[(2, 1)] = '>';
    let mut shortest_paths = vec![];
//    dbg!(code);
    let code = "A".to_string()+ original_code;
    for (first, second) in code.chars().zip(code.chars().skip(1)) {
        if first == second {
            shortest_paths.push(vec!["A".to_string()]);
            continue;
        }
        let start_pos = keypad_map.find_tile_pos(|tile| tile == &first).unwrap();
        let end_pos = keypad_map.find_tile_pos(|tile| tile == &second).unwrap();
        let dx = end_pos.0 - start_pos.0;
        let dy = end_pos.1 - start_pos.1;
        let xseg = match dx {
            0 => "",
            1 => ">",
            2 => ">>",
            -1 => "<",
            -2 => "<<",
            _ => unreachable!(),
        };
        let yseg = match dy {
            0 => "",
            1 => "v",
            -1 => "^",
            _ => unreachable!(),
        };
        if xseg.is_empty() || yseg.is_empty() {
            shortest_paths.push(vec![xseg.to_string()+yseg+"A"]);
        } else {
            if start_pos == (0,1) {
                shortest_paths.push(vec![xseg.to_string()+yseg+"A"]);
            } else if end_pos == (0,1) {
                shortest_paths.push(vec![yseg.to_string()+xseg+"A"]);
            } else {
                shortest_paths.push(vec![xseg.to_string()+yseg+"A", yseg.to_string()+xseg+"A"]);
            }
        }
    }
//    dbg!(&shortest_paths);
    let mut length = 0i64;
    if depth == 0 {
        for path in shortest_paths {
            length += path.iter().map(|s| s.len() as i64).min().unwrap();
        }
    } else {
        for path in shortest_paths {
            length += path.iter().map(|s| get_shortest_path_keypad(s, depth-1, memo_map)).min().unwrap();
        }
    }
//    dbg!(&code, depth);
    memo_map.insert((original_code.to_string(), depth), length);
    length as i64
}


pub fn part_two(input: &str) -> Option<u64> {
    let codes = input.lines().collect::<Vec<_>>();
    let mut result = 0;
    let mut memo_map = HashMap::default();
    for code in codes {
        //dbg!(code);
        let length = get_shortest_path_length_numeric(code, 25, &mut memo_map);
        let ncode: i64 = code[0..3].parse().unwrap();
        //        dbg!(ncode);
        //        dbg!(length);
        result += length * ncode;
    }
    Some(result as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
