use std::collections::{HashMap, HashSet};

advent_of_code::solution!(23);

pub fn part_one(input: &str) -> Option<usize> {
    let mut connections: HashMap<String, HashSet<String>> = Default::default();
    for line in input.lines() {
        let a = line[0..2].to_string();
        let b = line[3..5].to_string();
        //        println!("{a} -> {b}");
        connections.entry(a.clone()).or_default().insert(b.clone());
        connections.entry(b).or_default().insert(a);
    }
    let mut triples: HashSet<Vec<String>> = Default::default();
    for (key, values) in &connections {
        for value in values {
            let Some(value_connections) = connections.get(value) else {
                continue;
            };
            let shared_third_parties = values.intersection(value_connections);
            for third_party in shared_third_parties {
                if key.starts_with("t") || value.starts_with("t") || third_party.starts_with("t") {
                    let mut triple = vec![key.clone(), value.clone(), third_party.clone()];
                    triple.sort();
                    triples.insert(triple);
                }
            }
        }
    }
    //    dbg!(&triples);
    //    dbg!(triples.len());
    Some(triples.len())
}

#[derive(Debug)]
struct Entry {
    in_clique: HashSet<String>,
    out_clique: HashSet<String>,
    to_test: HashSet<String>,
}

pub fn part_two(input: &str) -> Option<String> {
    let mut connections: HashMap<String, HashSet<String>> = Default::default();
    for line in input.lines() {
        let a = line[0..2].to_string();
        let b = line[3..5].to_string();
        connections.entry(a.clone()).or_default().insert(b.clone());
        connections.entry(b).or_default().insert(a);
    }
    let mut todo = vec![Entry { in_clique: HashSet::new(), out_clique: HashSet::new(), to_test: connections.keys().to_owned().cloned().collect() }];
    let mut result: Vec<String> = Vec::new();
    let empty_set = HashSet::new();
    let mut maximal_length = 0;
    while let Some(mut entry) = todo.pop() {
        //        dbg!(&entry);
        while let Some(candidate) = entry.to_test.iter().cloned().next() {
            //            dbg!(&candidate);
            let mut in_clique = entry.in_clique.clone();
            in_clique.insert(candidate.clone());
            let neighbours = connections.get(&candidate).unwrap_or(&empty_set);
            //            dbg!(&neighbours);
            let out_clique: HashSet<String> = entry.out_clique.intersection(neighbours).cloned().collect();
            let to_test: HashSet<String> = entry.to_test.intersection(neighbours).cloned().collect();
            //            dbg!(&out_clique);
            //            dbg!(&to_test);
            if to_test.is_empty() && out_clique.is_empty() {
                if in_clique.len() > maximal_length {
//                    if in_clique.iter().any(|s| s.starts_with("t")) {
                        maximal_length = in_clique.len();
                        result = in_clique.into_iter().collect();
                    //}
                }
                //println!("Maximal: {:?}", in_clique);
            } else {
                todo.push(Entry { in_clique, out_clique, to_test });
            }
            entry.to_test.remove(&candidate);
            entry.out_clique.insert(candidate);
        };
    }

    /*    let mut max_shared_third_parties = 0;

        for (key, values) in &connections {
            for value in values {
                let Some(value_connections) = connections.get(value) else {
                    continue;
                };
                let shared_third_parties: Vec<_> = values.intersection(value_connections).collect();
                if key.starts_with("t") || value.starts_with("t") || shared_third_parties.iter().any(|s| s.starts_with("t")) {
                    if shared_third_parties.len() > max_shared_third_parties {
                        max_shared_third_parties = shared_third_parties.len();
                        result = shared_third_parties.iter().map(|s| s.to_string()).collect();
                        result.push(key.to_string());
                        result.push(value.to_string());
                    }
                }
            }
        }
        //    dbg!(&triples);
        //    dbg!(triples.len());*/
    result.sort();
    Some(result.join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
