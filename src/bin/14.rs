//use image::{GrayImage, Luma};
//use image::codecs::png::PngEncoder;
use unscanny::Scanner;
use advent_of_code::map::{Map, Scalar};

advent_of_code::solution!(14);

#[derive(Debug, Clone, Copy)]
struct Tile {
    count: i64,
}

impl From<&Tile> for char {
    fn from(value: &Tile) -> Self {
        if value.count == 0 {
            '.'
        } else {
            ('0' as i64 + value.count) as u8 as char
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let seconds_to_simulate = 100;
    let mut s = Scanner::new(input);
    //    let mut machines = vec![];
    let width = s.eat_while(char::is_ascii_digit).parse::<i64>().unwrap();
    let w2 = width / 2;
    s.expect("\n");
    let height = s.eat_while(char::is_ascii_digit).parse::<i64>().unwrap();
    let h2 = height / 2;
    s.expect("\n");
    let mut quadrant_counts = vec![0, 0, 0, 0];
    let mut map = Map::with_unknown_height(width as usize, Tile { count: 0 });
    for y in 0..height {
        for x in 0..width {
            map[(x as Scalar, y as Scalar)] = Tile { count: 0 };
        }
    }
    while !s.done() {
        s.expect("p=");
        let x = s.eat_while(char::is_ascii_digit).parse::<i64>().unwrap();
        s.expect(",");
        let y = s.eat_while(char::is_ascii_digit).parse::<i64>().unwrap();

        s.expect(" v=");
        let mut vx = s.eat_while(|c: &char| *c == '-' || char::is_ascii_digit(c)).parse::<i64>().unwrap();
        if vx < 0 {
            vx += width;
        }
        s.expect(",");
        let mut vy = s.eat_while(|c: &char| *c == '-' || char::is_ascii_digit(c)).parse::<i64>().unwrap();
        if vy < 0 {
            vy += height;
        }
        s.expect("\n");
        let x = (seconds_to_simulate * vx + x) % width;
        let y = (seconds_to_simulate * vy + y) % height;
//        let x = (x+ width) % width;
//        let y = (y+ height) % height;
//        dbg!(x,y);
        map[(x as Scalar,y as Scalar)].count += 1;
        if x < w2 {
            if y < h2 {
                quadrant_counts[0] += 1;
            } else if y > h2 {
                quadrant_counts[1] += 1;
            }
        } else if x > w2 {
            if y < h2 {
                quadrant_counts[2] += 1;
            } else if y > h2 {
                quadrant_counts[3] += 1;
            }
        }
    }
//    dbg!(&quadrant_counts);
//    map.print_map();
    let result = quadrant_counts[0] * quadrant_counts[1] * quadrant_counts[2] * quadrant_counts[3];
    Some(result)
}

struct Drone {
    x: i64,
    y: i64,
    vx: i64,
    vy: i64,
}

pub fn part_two(input: &str) -> Option<u32> {
    let seconds_to_simulate = 100;
    let mut s = Scanner::new(input);
    //    let mut machines = vec![];
    let width = s.eat_while(char::is_ascii_digit).parse::<i64>().unwrap();
    let w2 = width / 2;
    s.expect("\n");
    let height = s.eat_while(char::is_ascii_digit).parse::<i64>().unwrap();
    let h2 = height / 2;
    s.expect("\n");
    let mut quadrant_counts = vec![0, 0, 0, 0];
    let mut map = Map::with_unknown_height(width as usize, Tile { count: 0 });
    for y in 0..height {
        for x in 0..width {
            map[(x as Scalar, y as Scalar)] = Tile { count: 0 };
        }
    }
    let mut drones = vec![];
    while !s.done() {
        s.expect("p=");
        let x = s.eat_while(char::is_ascii_digit).parse::<i64>().unwrap();
        s.expect(",");
        let y = s.eat_while(char::is_ascii_digit).parse::<i64>().unwrap();

        s.expect(" v=");
        let mut vx = s.eat_while(|c: &char| *c == '-' || char::is_ascii_digit(c)).parse::<i64>().unwrap();
        if vx < 0 {
            vx += width;
        }
        s.expect(",");
        let mut vy = s.eat_while(|c: &char| *c == '-' || char::is_ascii_digit(c)).parse::<i64>().unwrap();
        if vy < 0 {
            vy += height;
        }
        s.expect("\n");
        drones.push(Drone { x, y, vx, vy });
    }
    /*
    std::fs::create_dir_all("images").unwrap();
    for i in 2000..10000 {
        let mut img = GrayImage::new(width as u32, height as u32);
        for drone in &drones {
            let x = (i * drone.vx + drone.x) % width;
            let y = (i * drone.vy + drone.y) % height;
            img.put_pixel(x as u32, y as u32, Luma([255]));
        }
        img.write_with_encoder(PngEncoder::new(File::create(format!("images/drone_{:06}.png", i)).unwrap())).unwrap();

    }*/
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
