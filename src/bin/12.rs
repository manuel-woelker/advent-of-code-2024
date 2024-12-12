use std::collections::{HashSet, VecDeque};
use advent_of_code::map::{Map, Scalar};

advent_of_code::solution!(12);

#[derive(Debug, Clone)]
struct Tile {
    crop: char,
    visited: bool,
}

impl Default for Tile {
    fn default() -> Self {
        Tile { crop: '.', visited: false }
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        Tile { crop: c, visited: false }
    }
}

impl From<&Tile> for char {
    fn from(tile: &Tile) -> Self {
        tile.crop
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = Map::<Tile>::parse_ascii(input);
    //    map.print_map();
    let mut result = 0;
    for y in 0..map.get_height() {
        for x in 0..map.width {
            let tile = &mut map[(x as Scalar, y as Scalar)];
            if tile.visited {
                continue;
            }
            let mut area = 0;
            let mut perimeter = 0;
            let crop = tile.crop;
            let mut todo = vec![(x as i32, y as i32)];
            while let Some((x, y)) = todo.pop() {
                let tile = &mut map[(x, y)];
                if tile.visited {
                    continue;
                }
                tile.visited = true;
                area += 1;
                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    if map.is_in_bounds(x + dx, y + dy) {
                        let tile = &mut map[(x + dx, y + dy)];
                        if tile.crop == crop {
                            todo.push((x + dx, y + dy));
                        } else {
                            perimeter += 1;
                        }
                    } else {
                        perimeter += 1;
                    }
                }
            }
            //            println!("{}: {} ({}x{})", crop, perimeter*area, area, perimeter);
            result += perimeter * area;
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = Map::<Tile>::parse_ascii(input);
//    map.print_map();
    let mut result = 0;
    for y in 0..map.get_height() {
        for x in 0..map.width {
            let tile = &mut map[(x as Scalar, y as Scalar)];
            if tile.visited {
                continue;
            }
            let mut area = 0;
            let mut perimeter = 0;
            let crop = tile.crop;
            let mut todo = VecDeque::from([(x as i32, y as i32)]);
            let mut sides: HashSet<(i32, i32, i32)> = HashSet::new();
            let mut counts = [0, 0, 0, 0, 0];
            while let Some((x, y)) = todo.pop_back() {
                let tile = &mut map[(x, y)];
                if tile.visited {
                    continue;
                }
                tile.visited = true;
                area += 1;
                for (dx, dy, dir) in [(-1, 0, 1), (1, 0, 2), (0, -1, 3), (0, 1, 4)] {
                    if map.is_in_bounds(x + dx, y + dy) {
                        let tile = &mut map[(x + dx, y + dy)];
                        if tile.crop == crop {
                            todo.push_back((x + dx, y + dy));
                            continue;
                        }
                    }
                    sides.insert((x, y, dir));
                    let mut found_sides = 0i32;
                    for (sx, sy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                        if sides.contains(&(x + sx, y + sy, dir)) {
                            // Side already visited, do not increase perimeter
                            found_sides += 1;
                        }
                    }
                    //dbg!(found_sides);
                    // New side, increase perimeter
                    perimeter += 1 - found_sides;
                    counts[dir as usize] += 1;
                    if dir == 3 {
//                        dbg!((x, y, dir));
                    }
                    //dbg!((x, y, dir));
                }
            }
//            println!("{}: {} ({}x{})", crop, perimeter*area, area, perimeter);
//            dbg!(counts);
            result += perimeter * area;
        }
    }
    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }

    #[test]
    fn test_part_two_simple_1() {
        let result = part_two(r#"A"#);
        assert_eq!(result, Some(4));
    }
    #[test]
    fn test_part_two_simple_2a() {
        let result = part_two(r#"AA"#);
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two_simple_3() {
        let result = part_two(r#"AA
AA"#);
        assert_eq!(result, Some(16));
    }

    #[test]
    fn test_part_two_example_1() {
        let result = part_two(r#"AAAA
BBCD
BBCC
EEEC"#);
        assert_eq!(result, Some(80));
    }

    #[test]
    fn test_part_two_example_2() {
        let result = part_two(r#"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"#);
        assert_eq!(result, Some(436));
    }

    #[test]
    fn test_part_two_example_3() {
        let result = part_two(r#"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"#);
        assert_eq!(result, Some(236));
    }

    #[test]
    fn test_part_two_example_4() {
        let result = part_two(r#"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"#);
        assert_eq!(result, Some(368));
    }
}
