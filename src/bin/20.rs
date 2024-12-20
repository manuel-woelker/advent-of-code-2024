use std::collections::{BinaryHeap, HashSet};
use advent_of_code::map::{Map, Scalar};

advent_of_code::solution!(20);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum TileKind {
    Free,
    Wall,
    Start,
    End,
}

#[derive(Debug, Copy, Clone)]
struct Tile {
    kind: TileKind,
    distance_to_end: u64,
}

impl Default for Tile {
    fn default() -> Self {
        Tile { kind: TileKind::Free, distance_to_end: u64::MAX }
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile { kind: TileKind::Free, distance_to_end: u64::MAX },
            '#' => Tile { kind: TileKind::Wall, distance_to_end: u64::MAX },
            'S' => Tile { kind: TileKind::Start, distance_to_end: u64::MAX },
            'E' => Tile { kind: TileKind::End, distance_to_end: u64::MAX },
            _ => panic!(),
        }
    }
}

impl From<&Tile> for char {
    fn from(tile: &Tile) -> Self {
        match tile.kind {
            TileKind::Free => '.',
            TileKind::Wall => '#',
            TileKind::Start => 'S',
            TileKind::End => 'E',
        }
    }
}

#[derive(PartialEq, Eq)]
struct Entry {
    pos: (Scalar, Scalar),
    distance: u64,
    score: u64,
}


impl Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct Path {
    pos: (Scalar, Scalar),
    visited: HashSet<(Scalar, Scalar)>,
    distance: u64,
}

pub fn part_one(input: &str) -> Option<u32> {
    let (input_first, input_rest) = input.split_once("\n").unwrap();
    let min_cheat_improvement: i64 = input_first.parse().unwrap();
    let mut map: Map<Tile> = Map::parse_ascii(input_rest);
//    map.print_map();

    let mut start = (0, 0);
    let mut end = (0, 0);
    for y in 0..map.get_height() {
        for x in 0..map.width {
            let tile = &mut map[(x as Scalar, y as Scalar)];
            if tile.kind == TileKind::Start {
                start = (x as Scalar, y as Scalar);
            } else if tile.kind == TileKind::End {
                tile.distance_to_end = 0;
                end = (x as Scalar, y as Scalar);
            }
        }
    }

    // Compute distances to end
    let mut todo = vec![end];
    let mut visited = HashSet::new();
    while let Some(pos) = todo.pop() {
        if !visited.insert(pos) {
            continue;
        }
        let (x, y) = pos;
        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let nx = x + dx;
            let ny = y + dy;
            if !map.is_in_bounds(nx, ny) || map[(nx, ny)].kind == TileKind::Wall {
                continue;
            }
            map[(nx, ny)].distance_to_end = map[(nx, ny)].distance_to_end.min(map[(x, y)].distance_to_end + 1);
            todo.push((nx, ny));
        }
    }
    let shortest_path = map[start].distance_to_end;
    println!("Shortest path is {}", map[start].distance_to_end);
    let maximum_distance = (shortest_path as i64 - min_cheat_improvement) as u64;
    println!("Maximum distance is {}", maximum_distance);

    let mut todo = vec![Path { pos: start, visited: HashSet::new(), distance: 0 }];
    let mut found_paths = 0;
    while let Some(mut path) = todo.pop() {
        if path.pos == end {
            found_paths += 1;
            panic!("Should not happen");
            continue;
        }
        path.visited.insert(path.pos);
        let (x, y) = path.pos;
        let distance = path.distance + 1;
        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let nx = x + dx;
            let ny = y + dy;
            if !map.is_in_bounds(nx, ny) {
                continue;
            }
            if path.visited.contains(&(nx, ny)) {
                continue;
            }
            if map[(nx, ny)].kind == TileKind::Wall {
                // Try cheating
                let nnx = nx + dx;
                let nny = ny + dy;
                if !map.is_in_bounds(nnx, nny) || map[(nnx, nny)].kind == TileKind::Wall {
                    continue;
                }
                let total_distance = distance + map[(nnx, nny)].distance_to_end +1;
                if total_distance <= maximum_distance {
                    found_paths += 1;
                }
                continue;
            }
            let score = distance + ((end.0 - nx).abs() + (end.1 - ny).abs()) as u64;
            if score > maximum_distance {
                continue;
            }
            todo.push(Path { pos: (nx, ny), visited: path.visited.clone(), distance });
        }
    }
    Some(found_paths)
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
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_part_one_11() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 11));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_one_12() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 12));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
