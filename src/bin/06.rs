use std::collections::HashSet;

advent_of_code::solution!(6);

#[derive(Clone)]
struct Tile {
    pub obstacle: bool,
    pub visited: bool,
    pub visited_directions: HashSet<Direction>,
}


#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone)]
struct Map {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Tile>,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut guard_x = 0;
    let mut guard_y = 0;
    let mut guard_dir = Direction::North;
    let mut x = 0;
    let mut y = 0;
    let mut tiles = vec![];
    let width = input.lines().next().unwrap().len();
    let mut height = 0;
    for line in input.lines() {
        height += 1;
        x = 0;
        for char in line.chars() {
            match char {
                '#' => {
                    let tile = Tile { obstacle: true, visited: false, visited_directions: HashSet::new() };
                    tiles.push(tile);
                },
                '.' => {
                    let tile = Tile { obstacle: false, visited: false, visited_directions: HashSet::new() };
                    tiles.push(tile);
                }
                '^' => {
                    let tile = Tile { obstacle: false, visited: true, visited_directions: HashSet::new() };
                    tiles.push(tile);
                    guard_x = x;
                    guard_y = y;
                }
                _ => {
                    panic!("unexpected character: {}", char);
                }
            }
            x+=1;
        }
        y+=1;
    }
    let mut map = Map {
        width,
        height,
        tiles,
    };

    let mut visited_tiles = 1;
    loop {
        let (next_x, next_y) = match guard_dir {
            Direction::North => {
                (guard_x, guard_y - 1)
            }
            Direction::East => {
                (guard_x + 1, guard_y)
            }
            Direction::South => {
                (guard_x, guard_y + 1)
            }
            Direction::West => {
                (guard_x - 1, guard_y)
            }
        };
        if (next_x < 0 || next_x >= map.width) || (next_y < 0 || next_y >= map.height) {
            break;
        }

        let next_tile = &mut map.tiles[next_y * map.width + next_x];
        if next_tile.obstacle {
            guard_dir = match guard_dir {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            }
        } else {
            if !next_tile.visited {
                visited_tiles += 1;
                next_tile.visited = true;
            }
            guard_x = next_x;
            guard_y = next_y;
        }

    }

    Some(visited_tiles)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut guard_x = 0isize;
    let mut guard_y = 0isize;
    let mut x = 0;
    let mut y = 0;
    let mut tiles = vec![];
    let width = input.lines().next().unwrap().len();
    let mut height = 0;
    for line in input.lines() {
        height += 1;
        x = 0;
        for char in line.chars() {
            match char {
                '#' => {
                    let tile = Tile { obstacle: true, visited: false, visited_directions: HashSet::new() };
                    tiles.push(tile);
                },
                '.' => {
                    let tile = Tile { obstacle: false, visited: false, visited_directions: HashSet::new() };
                    tiles.push(tile);
                }
                '^' => {
                    let tile = Tile { obstacle: false, visited: true, visited_directions: HashSet::new() };
                    tiles.push(tile);
                    guard_x = x;
                    guard_y = y;
                }
                _ => {
                    panic!("unexpected character: {}", char);
                }
            }
            x+=1;
        }
        y+=1;
    }
    let mut map = Map {
        width,
        height,
        tiles,
    };
    let original_map = map.clone();
    let original_guard_x = guard_x;
    let original_guard_y = guard_y;
    let mut visited_tiles = HashSet::new();
    {
        let mut guard_dir = Direction::North;
        loop {
            let (next_x, next_y) = match guard_dir {
                Direction::North => {
                    (guard_x, guard_y - 1)
                }
                Direction::East => {
                    (guard_x + 1, guard_y)
                }
                Direction::South => {
                    (guard_x, guard_y + 1)
                }
                Direction::West => {
                    (guard_x - 1, guard_y)
                }
            };
            if (next_x < 0 || next_x >= map.width as isize) || (next_y < 0 || next_y >= map.height as isize) {
                break;
            }

            let next_tile = &mut map.tiles[(next_y * map.width as isize + next_x) as usize];
            if next_tile.obstacle {
                guard_dir = match guard_dir {
                    Direction::North => Direction::East,
                    Direction::East => Direction::South,
                    Direction::South => Direction::West,
                    Direction::West => Direction::North,
                }
            } else {
                if !next_tile.visited {
                    visited_tiles.insert((next_x, next_y));
                    next_tile.visited = true;
                }
                guard_x = next_x;
                guard_y = next_y;
            }

        }
    }

    let mut found_loops = 0u32;

    for (new_x, new_y) in visited_tiles {
        dbg!(new_x, new_y);
        let mut map = original_map.clone();
        map.tiles[new_y as usize * map.width + new_x as usize].obstacle = true;
        let mut seen_states = HashSet::new();
        let mut guard_dir = Direction::North;
        let mut guard_x = original_guard_x;
        let mut guard_y = original_guard_y;
        loop {
            let (next_x, next_y) = match guard_dir {
                Direction::North => {
                    (guard_x, guard_y - 1)
                }
                Direction::East => {
                    (guard_x + 1, guard_y)
                }
                Direction::South => {
                    (guard_x, guard_y + 1)
                }
                Direction::West => {
                    (guard_x - 1, guard_y)
                }
            };
            if (next_x < 0 || next_x >= map.width as isize) || (next_y < 0 || next_y >= map.height as isize) {
                break;
            }

            let next_tile = &mut map.tiles[next_y as usize * map.width + next_x as usize];
            if next_tile.obstacle {
                guard_dir = match guard_dir {
                    Direction::North => Direction::East,
                    Direction::East => Direction::South,
                    Direction::South => Direction::West,
                    Direction::West => Direction::North,
                }
            } else {
                if !next_tile.visited {
                    next_tile.visited = true;
                }
                guard_x = next_x;
                guard_y = next_y;
                let state = (guard_x, guard_y, guard_dir);
                if seen_states.contains(&state) {
                    found_loops += 1;
                    break;
                } else {
                    seen_states.insert(state);
                }
            }

        }

    }

    Some(found_loops)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
