use std::collections::{HashSet, VecDeque};
use advent_of_code::map::{Map, Scalar};

advent_of_code::solution!(18);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TileKind {
    Free,
    Wall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Tile {
    kind: TileKind,
    is_shortest_path: bool,
}


impl Default for Tile {
    fn default() -> Self {
        Tile { kind: TileKind::Free, is_shortest_path: false }
    }
}

impl From<&Tile> for char {
    fn from(tile: &Tile) -> Self {
        if tile.is_shortest_path {
            return 'O';
        }
        match tile.kind {
            TileKind::Free => '.',
            TileKind::Wall => '#',
        }
    }
}

struct Path {
    visited: HashSet<(Scalar, Scalar)>,
    pos: (Scalar, Scalar),
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let size: usize = lines.next().unwrap().parse::<usize>().unwrap()+1;
    let bytes: usize = lines.next().unwrap().parse::<usize>().unwrap();
    let mut map: Map<Tile> = Map::new(size, size);
    for line in lines.take(bytes) {
        let (xs, ys) = line.split_once(',').unwrap();
        let x: Scalar = xs.parse().unwrap();
        let y: Scalar = ys.parse().unwrap();
        map[(x, y)].kind = TileKind::Wall;
    }
    map.print_map();

    let start = (0 as Scalar, 0 as Scalar);
    let end = ((size-1) as Scalar, (size-1) as Scalar);
    let mut todo = VecDeque::from([Path { visited: HashSet::new(), pos: start }]);
    while let Some(mut path) = todo.pop_front() {
        let (x, y) = path.pos;
        if (x, y) == end {
            for (x, y) in &path.visited {
                map[(*x, *y)].is_shortest_path = true;
            }
            map[end].is_shortest_path = true;
            println!("\nMap:\n");
            map.print_map();
            return Some(path.visited.len() as u32);
        }
        let tile = &mut map[(x, y)];
        if tile.kind == TileKind::Wall {
            continue;
        }
        path.visited.insert((x, y));
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (nx, ny) = (x + dx, y + dy);
            if !map.is_in_bounds(nx, ny) || map[(nx, ny)].kind == TileKind::Wall {
                continue;
            }
            if path.visited.contains(&(nx, ny)) {
                continue;
            }
            todo.push_back(Path { visited: path.visited.clone(), pos: (nx, ny) });
        }
    }
    None
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
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
