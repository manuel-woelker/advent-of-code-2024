use advent_of_code::map::{Map, Scalar, ToChar};

advent_of_code::solution!(10);


#[derive(Debug, Copy, Clone)]
struct Tile {
    height: u32,
    visited: bool,
}

impl Default for Tile {
    fn default() -> Self {
        Tile { height: 99, visited: false }
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        Tile { height: c as u32 - '0' as u32, visited: false }
    }
}

impl From<&Tile> for char {
    fn from(tile: &Tile) -> char {
        (tile.height as u8 + '0' as u8) as char
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut result = 0;
    let map = Map::<Tile>::parse_ascii(input);
//    map.print_map();
    for y in 0..map.get_height() {
        for x in 0..map.width {
            let tile = map[(x as Scalar, y as Scalar)];
            if tile.height != 0 {
                continue;
            }
            let mut tmap = map.clone();
            let count = count_trailheads(&mut tmap, x as i32, y as i32, 0);
            result += count;
        }
    }
    Some(result)
}

fn count_trailheads(map: &mut Map<Tile>, x: i32, y: i32, current_height: u32) -> u32 {
    let next_height = current_height + 1;
    let mut num_trailheads = 0;
    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        if map.is_in_bounds(x + dx, y + dy) {
            let tile = &mut map[(x + dx, y + dy)];
            if tile.height == next_height {
                if !tile.visited {
                    tile.visited = true;
                    if tile.height == 9 {
                        num_trailheads += 1;
                    } else if tile.height == next_height {
                        num_trailheads += count_trailheads(map, x + dx, y + dy, next_height);
                    }
                }
            }
        }
    }
    num_trailheads
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result = 0;
    let map = Map::<Tile>::parse_ascii(input);
    //    map.print_map();
    for y in 0..map.get_height() {
        for x in 0..map.width {
            let tile = map[(x as Scalar, y as Scalar)];
            if tile.height != 0 {
                continue;
            }
            let mut tmap = map.clone();
            let count = count_trailheads2(&mut tmap, x as i32, y as i32, 0);
            result += count;
        }
    }
    Some(result)
}

fn count_trailheads2(map: &mut Map<Tile>, x: i32, y: i32, current_height: u32) -> u32 {
    let next_height = current_height + 1;
    let mut num_trailheads = 0;
    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        if map.is_in_bounds(x + dx, y + dy) {
            let tile = &mut map[(x + dx, y + dy)];
            if tile.height == next_height {
                if !tile.visited {
                    if tile.height == 9 {
                        num_trailheads += 1;
                    } else if tile.height == next_height {
                        num_trailheads += count_trailheads2(map, x + dx, y + dy, next_height);
                    }
                }
            }
        }
    }
    num_trailheads
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
