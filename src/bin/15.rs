use std::collections::VecDeque;
use advent_of_code::map::Map;
use crate::WideTile::Wall;

advent_of_code::solution!(15);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    Free,
    Wall,
    Box,
    Robot,
}


#[derive(Clone, Copy, Debug)]
struct Tile {
    kind: Kind,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '#' => Self { kind: Kind::Wall },
            '.' => Self { kind: Kind::Free },
            '@' => Self { kind: Kind::Robot },
            'O' => Self { kind: Kind::Box },
            _ => panic!(),
        }
    }
}

impl From<&Tile> for char {
    fn from(value: &Tile) -> Self {
        match value.kind {
            Kind::Free => '.',
            Kind::Wall => '#',
            Kind::Box => 'O',
            Kind::Robot => '@',
        }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Tile { kind: Kind::Wall }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (map_input, move_input) = input.split_once("\n\n").unwrap();
    let mut map = Map::parse_ascii(map_input);
    let mut rx = 0i32;
    let mut ry = 0i32;
    for x in 0..map.width {
        for y in 0..map.get_height() {
            let tile: &mut Tile = &mut map[(x,y)];
            if tile.kind == Kind::Robot {
                rx = x as i32;
                ry = y as i32;
                break;
            }
        }
    }
    map.print_map();
    for c in move_input.chars() {
        let mut dx = 0i32;
        let mut dy = 0i32;
        match c {
            '>' => dx = 1,
            '<' => dx = -1,
            'v' => dy = 1,
            '^' => dy = -1,
            '\n' => continue,
            _ => panic!(),
        };
        let mut cx = rx + dx;
        let mut cy = ry + dy;
        let mut moved_boxes = 0;
        let mut can_move = false;
        loop {
            let kind = map[(cx, cy)].kind;
            match kind {
                Kind::Free => {
                    can_move = true;
                    break;
                }
                Kind::Wall => break,
                Kind::Box => {
                    moved_boxes += 1;
                    cx += dx;
                    cy += dy;
                }
                Kind::Robot => {
                    panic!("Robot should not be here");
                },
            }
        }
        if can_move {
            map[(rx, ry)] = Tile { kind: Kind::Free };
            let mut cx = rx + dx;
            let mut cy = ry + dy;
            rx = cx;
            ry = cy;
            map[(rx, ry)] = Tile { kind: Kind::Robot };
            for _ in 0..moved_boxes {
                cx += dx;
                cy += dy;
                map[(cx, cy)].kind = Kind::Box;
            }
        }

//        println!("\nMove: {}", c);
//        map.print_map();
    }
    map.print_map();
    Some(compute_score(&map))
}

fn compute_score(map: &Map<Tile>) -> u32 {
    let mut score = 0;
    for x in 0..map.width {
        for y in 0..map.get_height() {
            let tile: &Tile = &map[(x,y)];
            if tile.kind == Kind::Box {
                score += y * 100 + x;
            }
        }
    }
    score as u32
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WideTile {
    Free,
    Wall,
    BoxLeft,
    BoxRight,
    Robot,
}

impl From<&WideTile> for char {
    fn from(value: &WideTile) -> Self {
        match value {
            WideTile::Free => '.',
            WideTile::Wall => '#',
            WideTile::BoxLeft => '[',
            WideTile::BoxRight => ']',
            WideTile::Robot => '@',
        }
    }
}

impl Default for WideTile {
    fn default() -> Self {
        WideTile::Wall
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map_input, move_input) = input.split_once("\n\n").unwrap();
    let mut skinny_map = Map::parse_ascii(map_input);
    let mut map = Map::with_unknown_height(skinny_map.width*2, WideTile::default());
    for y in 0..skinny_map.get_height() {
        for x in 0..skinny_map.width {
            let tile: &Tile = &skinny_map[(x, y)];
            let new_tiles = match tile.kind {
                Kind::Free => (WideTile::Free, WideTile::Free),
                Kind::Wall => (WideTile::Wall, WideTile::Wall),
                Kind::Box => (WideTile::BoxLeft, WideTile::BoxRight),
                Kind::Robot => (WideTile::Robot, WideTile::Free),
            };
            map[(x*2, y)] = new_tiles.0;
            map[(x*2+1, y)] = new_tiles.1;
        }
    }
    let mut rx = 0i32;
    let mut ry = 0i32;
    for x in 0..map.width {
        for y in 0..map.get_height() {
            let tile = &mut map[(x, y)];
            if *tile == WideTile::Robot {
                rx = x as i32;
                ry = y as i32;
                break;
            }
        }
    }
    map.print_map();
    for c in move_input.chars() {
        let mut dx = 0i32;
        let mut dy = 0i32;
        match c {
            '>' => dx = 1,
            '<' => dx = -1,
            'v' => dy = 1,
            '^' => dy = -1,
            '\n' => continue,
            _ => panic!(),
        };
        let mut cx = rx + dx;
        let mut cy = ry + dy;
        let mut can_move = true;
        let mut new_map = map.clone();
        let mut test_stack = VecDeque::from([(cx, cy)]);
        let mut map_changes = Vec::new();
        while let Some((x, y)) = test_stack.pop_front() {
            let kind = map[(x, y)];
//            dbg!(x,y,kind);
            match kind {
                WideTile::Free => {
                }
                WideTile::Wall => {
                    can_move = false;
                    break
                },
                WideTile::BoxLeft => {
                    if dx == 0 {
                        // Move up/down
                        map_changes.push((x+dx, y+dy, WideTile::BoxLeft));
                        map_changes.push((x+dx+1, y+dy, WideTile::BoxRight));
                        map_changes.push((x,y, WideTile::Free));
                        map_changes.push((x+1,y, WideTile::Free));
                        test_stack.push_back((x + dx, y + dy));
                        test_stack.push_back((x + dx+1, y + dy));

                    } else {
                        // Move left/right
                        new_map[(x+dx, y+dy)] = WideTile::BoxLeft;
                        test_stack.push_back((x + dx, y + dy));
                    }
                },
                WideTile::BoxRight => {
                    if dx == 0 {
                        // Move up/down
                        map_changes.push((x+dx-1, y+dy, WideTile::BoxLeft));
                        map_changes.push((x+dx, y+dy, WideTile::BoxRight));
                        map_changes.push((x-1,y, WideTile::Free));
                        map_changes.push((x,y, WideTile::Free));
                        test_stack.push_back((x + dx-1, y + dy));
                        test_stack.push_back((x + dx, y + dy));

                    } else {
                        // Move left/right
                        new_map[(x+dx, y+dy)] = WideTile::BoxRight;
                        test_stack.push_back((x + dx, y + dy));
                    }
                },
                WideTile::Robot => {
                    panic!("Robot should not be here");
                },
            }
        }
        if can_move {
            while let Some((x, y, kind)) = map_changes.pop() {
                new_map[(x, y)] = kind;
            }
            new_map[(rx, ry)] = WideTile::Free;
            new_map[(cx, cy)] = WideTile::Robot;
            map = new_map;
            rx += dx;
            ry += dy;
        }

//        println!("\nMove: {}", c);


    }
    println!();
    map.print_map();
    let mut score = 0;
    for x in 0..map.width {
        for y in 0..map.get_height() {
            let tile = &map[(x,y)];
            if *tile == WideTile::BoxLeft {
                score += y * 100 + x;
            }
        }
    }
    // Wrong: 1443518
    Some(score as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_one_b() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }

    #[test]
    fn test_part_two_b() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 3));
        assert_eq!(result, Some(618));
    }
}
