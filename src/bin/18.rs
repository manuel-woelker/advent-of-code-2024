use std::collections::{BinaryHeap, HashSet, VecDeque};
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
    score: u64,
}


impl Default for Tile {
    fn default() -> Self {
        Tile { kind: TileKind::Free, is_shortest_path: false, score: u64::MAX }
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
    let mut heap = BinaryHeap::new();
    heap.push(Entry { pos: start, distance: 0, score: 0 });
    while let Some(entry) = heap.pop() {
        if entry.pos == end {
            return Some(entry.score as u32);
        }
        let (x, y) = entry.pos;
        let distance = entry.distance + 1;
        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let nx = x + dx;
            let ny = y + dy;
            if !map.is_in_bounds(nx, ny) || map[(nx, ny)].kind == TileKind::Wall {
                continue;
            }
            let score = distance + (end.0 - nx + end.1 - ny) as u64;
            if map[(nx, ny)].score > score {
                map[(nx, ny)].score = score;
                heap.push(Entry { pos: (nx, ny), distance, score });
            }
        }

    }

    None
}


pub fn part_one_bfs(input: &str) -> Option<u32> {
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

pub fn part_two(input: &str) -> Option<String> {
    let mut lines = input.lines();
    let size: usize = lines.next().unwrap().parse::<usize>().unwrap()+1;
    let min_bytes: usize = lines.next().unwrap().parse::<usize>().unwrap();
    let original_map: Map<Tile> = Map::new(size, size);
    let mut bytes = vec![];
    for line in lines {
        let (xs, ys) = line.split_once(',').unwrap();
        let x: Scalar = xs.parse().unwrap();
        let y: Scalar = ys.parse().unwrap();
        bytes.push((x, y));
    }
    let len = bytes.len();
    let mut failed_length = 0;
    'outer: for i in min_bytes..len {
//        dbg!(i);
        let mut map = original_map.clone();
        for (x, y) in &bytes[0..i] {
            map[(*x, *y)].kind = TileKind::Wall;
        }

        let start = (0 as Scalar, 0 as Scalar);
        let end = ((size - 1) as Scalar, (size - 1) as Scalar);
        let mut heap = BinaryHeap::new();
        heap.push(Entry { pos: start, distance: 0, score: 0 });
        while let Some(entry) = heap.pop() {
            if entry.pos == end {
                continue 'outer;
            }
            let (x, y) = entry.pos;
            let distance = entry.distance + 1;
            for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let nx = x + dx;
                let ny = y + dy;
                if !map.is_in_bounds(nx, ny) || map[(nx, ny)].kind == TileKind::Wall {
                    continue;
                }
                let score = distance + (end.0 - nx + end.1 - ny) as u64;
                if map[(nx, ny)].score > score {
                    map[(nx, ny)].score = score;
                    heap.push(Entry { pos: (nx, ny), distance, score });
                }
            }

        }
        failed_length = i;
        break;
    }
    let failed_byte = bytes[failed_length-1];
    Some(format!("{},{}", failed_byte.0, failed_byte.1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_bfs() {
        let result = part_one_bfs(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("6,1".to_string()));
    }
}
