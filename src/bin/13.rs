use unscanny::Scanner;

advent_of_code::solution!(13);

#[derive(Debug)]
struct Button {
    x: i64,
    y: i64,
}

#[allow(unused)]
#[derive(Debug)]
struct Machine {
    a: Button,
    b: Button,
    x: i64,
    y: i64,
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut s = Scanner::new(input);
//    let mut machines = vec![];
    let mut result = 0;
    while !s.done() {
        s.expect("Button A: X+");
        let x = s.eat_while(char::is_ascii_digit).parse::<i64>().unwrap();
        s.expect(", Y+");
        let y = s.eat_while(char::is_ascii_digit).parse::<i64>().unwrap();
        let a = Button { x, y };

        s.expect("\nButton B: X+");
        let x = s.eat_while(char::is_ascii_digit).parse::<i64>().unwrap();
        s.expect(", Y+");
        let y = s.eat_while(char::is_ascii_digit).parse::<i64>().unwrap();
        let b = Button { x, y };

        s.expect("\nPrize: X=");
        let x = s.eat_while(char::is_ascii_digit).parse::<i64>().unwrap();
        s.expect(", Y=");
        let y = s.eat_while(char::is_ascii_digit).parse::<i64>().unwrap();
        s.expect("\n\n");

        let mut minimum = i64::MAX;
        for a_pushes in 0..=100 {
            let x_left = x - a_pushes*a.x;
            let y_left = y - a_pushes*a.y;
            if x_left < 0 || y_left < 0 {
                continue;
            }
            if x_left % b.x != 0 || y_left % b.y != 0 {
                continue;
            }
            let b_pushes_for_x = x_left / b.x;
            let b_pushes_for_y = y_left / b.y;
            if b_pushes_for_y != b_pushes_for_x {
                continue;
            }
            minimum = minimum.min(a_pushes*3 + b_pushes_for_x);
        }
        if minimum != i64::MAX {
            result += minimum;
        }

    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut s = Scanner::new(input);
    //    let mut machines = vec![];
    let mut result = 0;
    while !s.done() {
        s.expect("Button A: X+");
        let x = s.eat_while(char::is_ascii_digit).parse::<i64>().unwrap();
        s.expect(", Y+");
        let y = s.eat_while(char::is_ascii_digit).parse::<i64>().unwrap();
        let a = Button { x, y };

        s.expect("\nButton B: X+");
        let x = s.eat_while(char::is_ascii_digit).parse::<i64>().unwrap();
        s.expect(", Y+");
        let y = s.eat_while(char::is_ascii_digit).parse::<i64>().unwrap();
        let b = Button { x, y };

        s.expect("\nPrize: X=");
        let x = s.eat_while(char::is_ascii_digit).parse::<i64>().unwrap()+10000000000000;
        s.expect(", Y=");
        let y = s.eat_while(char::is_ascii_digit).parse::<i64>().unwrap()+10000000000000;
        s.expect("\n\n");

        let mut minimum = i64::MAX;
        for a_inc in 0..1000 {
            for b_inc in 0..1000 {
                let x_inc = a_inc*a.x + b_inc*b.x;
                let y_inc = a_inc*a.y + b_inc*b.y;
                if x_inc != y_inc || x_inc == 0 {
                    continue;
                }
                //                    println!("{}", x_inc);
                let move_cost = a_inc*3 + b_inc;
                let moves = 10000000000000 / x_inc;
                let base_cost = moves*move_cost;

                let total_move = moves*x_inc;
                let y = y - total_move;
                let x = x - total_move;
//                dbg!((x,y));

                let maximum_pushes = 100000 / a.x.min(a.y).min(b.x).min(b.y);
                for a_pushes in 0..=maximum_pushes {
                    let x_left = x - a_pushes*a.x;
                    let y_left = y - a_pushes*a.y;
                    if x_left < 0 || y_left < 0 {
                        continue;
                    }
                    if x_left % b.x != 0 || y_left % b.y != 0 {
                        continue;
                    }
                    let b_pushes_for_x = x_left / b.x;
                    let b_pushes_for_y = y_left / b.y;
                    if b_pushes_for_y != b_pushes_for_x {
                        continue;
                    }
                    minimum = minimum.min(base_cost+a_pushes*3 + b_pushes_for_x);
                }

            }
        }
        //dbg!(minimum);
        if minimum != i64::MAX {
            result += minimum;
        }

    }
    // 74081777318586
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
