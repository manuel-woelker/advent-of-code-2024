use std::collections::{HashMap, HashSet, VecDeque};
use advent_of_code::map::{Map, Scalar};

advent_of_code::solution!(16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Free,
    Wall,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Tile {
    kind: Kind,
    c: char,
    lowest_score: u32,
    lowest_score_per_direction: HashMap<(i32, i32), u32>,
    predecessors: HashSet<(i32, i32)>,
}

const INITIAL_SCORE: u32 = u32::MAX;

impl Default for Tile {
    fn default() -> Self {
        Tile { kind: Kind::Wall, c: '#', lowest_score: INITIAL_SCORE, predecessors: Default::default(), lowest_score_per_direction: Default::default() }
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '#' => Self { kind: Kind::Wall, c, lowest_score: INITIAL_SCORE, predecessors: Default::default(), lowest_score_per_direction: Default::default() },
            '.' => Self { kind: Kind::Free, c, lowest_score: INITIAL_SCORE, predecessors: Default::default(), lowest_score_per_direction: Default::default() },
            'S' => Self { kind: Kind::Free, c, lowest_score: INITIAL_SCORE, predecessors: Default::default(), lowest_score_per_direction: Default::default() },
            'E' => Self { kind: Kind::Free, c, lowest_score: INITIAL_SCORE, predecessors: Default::default(), lowest_score_per_direction: Default::default() },
            _ => unreachable!(),
        }
    }
}

impl From<&Tile> for char {
    fn from(tile: &Tile) -> Self {
        tile.c
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = Map::<Tile>::parse_ascii(input);
    let mut start = (0, 0);
    let mut end = (0, 0);
    for y in 0..map.get_height() {
        for x in 0..map.width {
            let tile = &mut map[(x as Scalar, y as Scalar)];
            match tile.c {
                'S' => start = (x as i32, y as i32),
                'E' => end = (x as i32, y as i32),
                _ => (),
            };
        }
    }
    map[start].lowest_score = 0;

    let mut todo = vec![(start, (1, 0))];
    while let Some(((x, y), (dx, dy))) = todo.pop() {
        if (x, y) == end {
            continue;
        }
        let score = map[(x, y)].lowest_score;
        // straight
        if map.is_in_bounds(x + dx, y + dy) {
            let tile = &mut map[(x + dx, y + dy)];
            if tile.kind == Kind::Free && tile.lowest_score > score + 1 {
                tile.lowest_score = score + 1;
                todo.push(((x + dx, y + dy), (dx, dy)));
                tile.c = match (dx, dy) {
                    (1, 0) => '>',
                    (-1, 0) => '<',
                    (0, 1) => 'v',
                    (0, -1) => '^',
                    _ => unreachable!(),
                }
            }
        }
        // rotated
        for (rx, ry) in [(-1, 1), (1, -1)] {
            if map.is_in_bounds(x + dx, y + dy) {
                let ndx = dy * rx;
                let nx = x + ndx;
                let ndy = dx * ry;
                let ny = y + ndy;
                let tile = &mut map[(nx, ny)];
                if tile.kind == Kind::Free && tile.lowest_score > score + 1001 {
                    tile.lowest_score = score + 1001;
                    todo.push(((nx, ny), (ndx, ndy)));
                    tile.c = match (ndx, ndy) {
                        (1, 0) => '>',
                        (-1, 0) => '<',
                        (0, 1) => 'v',
                        (0, -1) => '^',
                        _ => unreachable!(),
                    }
                }
            }
        }
    }

    Some(map[end].lowest_score)
}

struct Path {
    score: u32,
    last_direction: (i32, i32),
    path: Vec<(i32, i32)>,
    visited: HashSet<(i32, i32)>,
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = Map::<Tile>::parse_ascii(input);
    let mut start = (0, 0);
    let mut end = (0, 0);
    for y in 0..map.get_height() {
        for x in 0..map.width {
            let tile = &mut map[(x as Scalar, y as Scalar)];
            match tile.c {
                'S' => start = (x as i32, y as i32),
                'E' => end = (x as i32, y as i32),
                _ => (),
            };
        }
    }

    let mut todo = VecDeque::from([Path {
        score: 0,
        last_direction: (1, 0),
        path: vec![start],
        visited: HashSet::from([start]),
    }]);
    let mut complete_paths = Vec::new();
    while let Some(path) = todo.pop_front() {
        let (x,y) = *path.path.last().unwrap();
        if  (x, y) == end {
            complete_paths.push(path);
            continue;
        }
        let score = path.score;
        // straight
        let (dx,dy) = path.last_direction;
        let nx = x + dx;
        let ny = y + dy;
        let tile = &mut map[(nx, ny)];
        if tile.kind == Kind::Free && *tile.lowest_score_per_direction.get(&(dx,dy)).unwrap_or(&INITIAL_SCORE) >= score + 1 {
            let next = (nx, ny);
            if !path.visited.contains(&next) {
                tile.lowest_score_per_direction.insert((dx, dy), score + 1);
                let mut next_path = path.path.clone();
                next_path.push((nx, ny));
                let mut next_visited = path.visited.clone();
                next_visited.insert((nx, ny));
                tile.lowest_score_per_direction.insert((dx, dy), score + 1);
                todo.push_back(Path {
                    score: score + 1,
                    last_direction: (dx, dy),
                    path: next_path,
                    visited: next_visited,
                });
            }
        }
        // rotated
        for (rx, ry) in [(-1, 1), (1, -1)] {
            let ndx = dy * rx;
            let nx = x + ndx;
            let ndy = dx * ry;
            let ny = y + ndy;
            let tile = &mut map[(nx, ny)];
            if tile.kind == Kind::Free && *tile.lowest_score_per_direction.get(&(ndx,ndy)).unwrap_or(&INITIAL_SCORE) >= score + 1001  {
                let next = (nx, ny);
                if !path.visited.contains(&next) {
                    tile.lowest_score_per_direction.insert((ndx, ndy), score + 1001);
                    let mut next_path = path.path.clone();
                    next_path.push((nx, ny));
                    let mut next_visited = path.visited.clone();
                    next_visited.insert((nx, ny));
                    todo.push_back(Path {
                        score: score + 1001,
                        last_direction: (ndx, ndy),
                        path: next_path,
                        visited: next_visited,
                    });
                }
            }
        }
    }
    let lowest_score = complete_paths.iter().map(|p| p.score).min().unwrap();
    for path in complete_paths {
        if path.score != lowest_score {
            continue;
        }
        for (x, y) in path.path {
            let tile = &mut map[(x, y)];
            tile.c = 'O';
        }
    }
    dbg!(lowest_score);
/*    while let Some((x, y)) = todo.pop() {
        let tile = &mut map[(x, y)];
        if tile.c != 'O' {
            tile.c = 'O';
            if tile.predecessors.len() > 1 {
                dbg!(x,y);
            }
            for (px, py) in tile.predecessors.iter() {
                todo.push((*px, *py))
            }
        }
    }*/
    map.print_map();

    let mut result = 0;
    for y in 0..map.get_height() {
        for x in 0..map.width {
            let tile = &mut map[(x as Scalar, y as Scalar)];
//            print!("{:5} ", tile.lowest_score_per_direction.values().min().unwrap_or(&99999));
            if tile.c == 'O' {
                result += 1
            };
        }
//        println!();
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }

    #[test]
    fn test_part_two_2() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(64));
    }
}
