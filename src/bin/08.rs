use std::collections::HashMap;
use std::ptr;
use advent_of_code::map::Map;

advent_of_code::solution!(8);

#[derive(Debug, Clone)]
struct Tile {
    antenna: Option<char>,
    is_antinode: bool,
}

#[derive(Debug, Clone)]
struct Position {
    frequency: char,
    x: i32,
    y: i32,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut antennas: HashMap<char, Vec<Position>> = HashMap::new();
    let mut map = Map::with_unknown_height(input.lines().next().unwrap().len(), Tile { antenna: None, is_antinode: false });
    let mut y= 0;
    let mut x = 0;
    for line in input.lines() {
        for char in line.chars() {
            let mut antenna = None;
            if char != '.' {
                antenna = Some(char);
                antennas.entry(char).or_default().push(Position { frequency: char, x, y });
            }
            map[(x, y)] = Tile { antenna, is_antinode: false };
            x += 1;
        }
        x = 0;
        y += 1;
    }
    // Compute antinodes for every pair of antennas
    let mut number_of_antinodes = 0;
    for frequency in antennas.values() {
        // iterate every pair of positions
        for pos1 in frequency {
            for pos2 in frequency {
                if ptr::eq(pos1, pos2) {
                    continue
                }
                let dx = pos1.x - pos2.x;
                let dy = pos1.y - pos2.y;
                let x = pos1.x + dx;
                let y = pos1.y + dy;
                if map.is_in_bounds(x,y) {
                    if !map[(x,y)].is_antinode {
                        map[(x, y)].is_antinode = true;
                        number_of_antinodes += 1;
                    }
                }
            }
        }
    }
//    map.print(&|tile| if tile.is_antinode { '#' } else {tile.antenna.unwrap_or('.')});
    Some(number_of_antinodes)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut antennas: HashMap<char, Vec<Position>> = HashMap::new();
    let mut map = Map::with_unknown_height(input.lines().next().unwrap().len(), Tile { antenna: None, is_antinode: false });
    let mut y= 0;
    let mut x = 0;
    for line in input.lines() {
        for char in line.chars() {
            let mut antenna = None;
            if char != '.' {
                antenna = Some(char);
                antennas.entry(char).or_default().push(Position { frequency: char, x, y });
            }
            map[(x, y)] = Tile { antenna, is_antinode: false };
            x += 1;
        }
        x = 0;
        y += 1;
    }
    // Compute antinodes for every pair of antennas
    let mut number_of_antinodes = 0;
    for frequency in antennas.values() {
        // iterate every pair of positions
        for pos1 in frequency {
            for pos2 in frequency {
                if ptr::eq(pos1, pos2) {
                    continue
                }
                let dx = pos1.x - pos2.x;
                let dy = pos1.y - pos2.y;
                let mut x = pos1.x;
                let mut y = pos1.y;
                while map.is_in_bounds(x,y) {
                    if !map[(x,y)].is_antinode {
                        map[(x, y)].is_antinode = true;
                        number_of_antinodes += 1;
                    }
                    x += dx;
                    y += dy;
                }
            }
        }
    }
    map.print(&|tile| if tile.is_antinode { '#' } else {tile.antenna.unwrap_or('.')});
    Some(number_of_antinodes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
