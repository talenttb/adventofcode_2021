use crate::days::libs::read_lines;
use std::collections::HashMap;
// usd std::iter;
// use itertools::Itertools;
// use std::collections::{HashMap, HashSet};

pub fn solution(part: &str, ff: bool) {
    let mut input = Vec::new();
    if ff {
        if let Ok(lines) = read_lines("./src/days/day14input") {
            for line in lines {
                if let Ok(l) = line {
                    input.push(l);
                }
            }
        }
    } else {
        input = vec![
            "NNCB".to_string(),
            "".to_string(),
            "CH -> B".to_string(),
            "HH -> N".to_string(),
            "CB -> H".to_string(),
            "NH -> C".to_string(),
            "HB -> C".to_string(),
            "HC -> B".to_string(),
            "HN -> C".to_string(),
            "NN -> C".to_string(),
            "BH -> H".to_string(),
            "NC -> B".to_string(),
            "NB -> B".to_string(),
            "BN -> B".to_string(),
            "BB -> N".to_string(),
            "BC -> B".to_string(),
            "CC -> N".to_string(),
            "CN -> C".to_string(),
        ];
    }

    match part {
        "1" => {
            let result = day14p1(&input);
            println!("Result: {}", result);
        }
        "2" => {
            let result = day14p2(&input);
            println!("Result: {}", result);
        }
        _ => (),
    }
}

fn convert_input(input: &[String]) -> (String, HashMap<String, char>) {
    // println!("{:?}", input);
    let polymer_template = (&input[0]).to_string();
    let (pair_insertion_rules, _): (Vec<_>, Vec<_>) =
        input.into_iter().partition(|&x| x.contains("->"));
    // println!("{:?},{:?}", polymer_template, pair_insertion_rules);
    let mut pair_rules = HashMap::new();
    for item in pair_insertion_rules {
        let x: Vec<&str> = item.split(" -> ").collect();
        pair_rules.insert(x[0].to_string(), x[1].chars().nth(0).unwrap());
    }
    (polymer_template, pair_rules)
}

fn day14p1(input: &[String]) -> i32 {
    let (template, rules) = convert_input(input);
    println!("Template: {:?}", template);

    let mut ch_list = template.chars().collect::<Vec<_>>();
    for _step in 1..=10 {
        let mut combine_list = vec![];
        let mut pair_list = vec![];
        for idx in 0..ch_list.len() - 1 {
            let term = String::from_iter(ch_list[idx..idx + 2].to_vec());
            // println!("{:?}", term);

            let ch = rules.get(&term).unwrap();
            // println!("{:?}", ch);
            pair_list.push(ch);
        }

        for i in 0..ch_list.len() {
            combine_list.push(ch_list[i]);
            if i < pair_list.len() {
                combine_list.push(*pair_list[i]);
            }
        }
        // println!(
        //     "After step {}:{:?}",
        //     _step,
        //     String::from_iter(combine_list.to_vec())
        // );
        ch_list = combine_list;
    }

    let mut ch_counter: HashMap<char, i32> = HashMap::new();

    for ch in ch_list {
        *ch_counter.entry(ch).or_insert(0) += 1;
        // match ch_counter.get(&ch) {
        //     Some(x) =>  *ch_counter.entry(*ch).or_insert(0) += 1;,
        //     None => return,
        // }
    }

    // println!("{:?}", ch_counter.values());
    let min_value = ch_counter.values().into_iter().min().unwrap();
    let max_value = ch_counter.values().into_iter().max().unwrap();

    max_value - min_value
}

fn day14p2(input: &[String]) -> u64 {
    let (template, rules) = convert_input(input);
    // println!("Template: {:?}", template);
    let iter = template.chars().collect::<Vec<char>>();
    let mut term_map: HashMap<(char, char), u64> = HashMap::new();
    for i in iter.windows(2) {
        *term_map.entry((i[0], i[1])).or_insert(0) += 1;
    }
    // println!("{:?}", term_map);
    for _ in 0..40 {
        let mut current = HashMap::<(char, char), u64>::new();
        for ((a, b), n) in term_map.iter() {
            // println!("({:?},{:?}) => {}", a, b, n);
            let term = String::from_iter(vec![a, b]);
            let ch = rules.get(&term).unwrap();
            // println!("{:?}", ch);
            *current.entry((*a, *ch)).or_insert(0) += n;
            *current.entry((*ch, *b)).or_insert(0) += n;
        }
        term_map = current;
    }
    // println!("{:?}", term_map);

    let counts = term_map
        .iter()
        .fold(HashMap::<char, u64>::new(), |mut m, (k, v)| {
            *m.entry(k.1).or_default() += v;
            m
        });
    let max = counts.iter().max_by_key(|(_, i)| *i).unwrap().1;
    let min = counts.iter().min_by_key(|(_, i)| *i).unwrap().1;
    println!("{:?},{:?}", max, min);
    max - min
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn day14p1v1() {
//         let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
//         let result = day14p1(&input);
//         let expect = 7;
//         assert_eq!(result, expect);
//     }
//     #[test]
//     fn day14p2v1() {
//         let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
//         let result = day14p2(&input);
//         let expect = 5;
//         assert_eq!(result, expect);
//     }
// }
