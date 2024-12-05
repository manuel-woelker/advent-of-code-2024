use std::collections::HashSet;

advent_of_code::solution!(5);

#[derive(Debug)]
struct Rule {
    before: u32,
    after: u32,
}

#[derive(Debug, Clone)]
struct Update {
    pages: Vec<u32>,
}

pub fn part_one(input: &str) -> Option<u32> {
    let (first, second) = input.split_once("\n\n").unwrap();
    let mut rules = vec![];
    for line in first.lines() {
        let (before, after) = line.split_once("|").unwrap();
        rules.push(Rule {before: before.parse().unwrap(), after: after.parse().unwrap()});
    }
    let mut updates = vec![];
    for line in second.lines() {
        let pages = line.split(",").map(|s| s.parse().unwrap()).collect();
        updates.push(Update {pages});
    }
    let mut result = 0;
    'outer: for update in &updates {
        for rule in &rules {
            // find in index of before in pages
            let before_pos = update.pages.iter().position(|&x| x == rule.before);
            let after_pos = update.pages.iter().position(|&x| x == rule.after);
            if let Some(before_pos) = before_pos {
                if let Some(after_pos) = after_pos {
                    if before_pos > after_pos {
                        continue 'outer;
                    }
                }
            }
        }
        // add middle page to result
        result += update.pages[update.pages.len() / 2];
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (first, second) = input.split_once("\n\n").unwrap();
    let mut rules = vec![];
    for line in first.lines() {
        let (before, after) = line.split_once("|").unwrap();
        rules.push(Rule {before: before.parse().unwrap(), after: after.parse().unwrap()});
    }
    let mut updates = vec![];
    let mut wrong_updates = vec![];
    for line in second.lines() {
        let pages = line.split(",").map(|s| s.parse().unwrap()).collect();
        updates.push(Update {pages});
    }
    let mut result = 0;
    'outer: for update in &updates {
        for rule in &rules {
            // find in index of before in pages
            let before_pos = update.pages.iter().position(|&x| x == rule.before);
            let after_pos = update.pages.iter().position(|&x| x == rule.after);
            if let Some(before_pos) = before_pos {
                if let Some(after_pos) = after_pos {
                    if before_pos > after_pos {
                        wrong_updates.push(update.clone());
                        continue 'outer;
                    }
                }
            }
        }
    }


    for update in &mut wrong_updates {
        let mut to_order: HashSet<u32> = HashSet::from_iter(update.pages.clone());
        let mut ordered: Vec<u32> = vec![];
        while !to_order.is_empty() {
            'inner: for candidate in &to_order {
                for rule in &rules {
                    if rule.after == *candidate {
                        if to_order.contains(&rule.before) {
                            //dbg!(rule.before);
                            continue 'inner;
                        }
                    }
                }
                ordered.push(*candidate);
                break;
            }
            to_order.remove(ordered.iter().last().unwrap());
        }
//        dbg!(&ordered);
        result += ordered[ordered.len() / 2];
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
