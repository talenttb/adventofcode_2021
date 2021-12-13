use crate::days::libs::read_lines;
// use std::collections::{HashMap, HashSet};
use std::collections::HashMap;

pub fn solution(part: &str, ff: bool) {
    let mut input = Vec::new();
    if ff {
        if let Ok(lines) = read_lines("./src/days/day12input") {
            for line in lines {
                if let Ok(l) = line {
                    // println!("{}", l);
                    input.push(l);
                    // input.push(l.parse::<i32>().unwrap());
                    // for v in l.split(",") {
                    //     input.push(v.parse::<i32>().unwrap());
                    // }
                }
            }
        }
    } else {
        // input = vec![
        //     "start-b".to_string(),
        //     "b-d".to_string(),
        //     "b-end".to_string(),
        // ];
        input = vec![
            "start-A".to_string(),
            "start-b".to_string(),
            "A-c".to_string(),
            "A-b".to_string(),
            "b-d".to_string(),
            "A-end".to_string(),
            "b-end".to_string(),
        ];
        // input = vec![
        //     "dc-end".to_string(),
        //     "HN-start".to_string(),
        //     "start-kj".to_string(),
        //     "dc-start".to_string(),
        //     "dc-HN".to_string(),
        //     "LN-dc".to_string(),
        //     "HN-end".to_string(),
        //     "kj-sa".to_string(),
        //     "kj-HN".to_string(),
        //     "kj-dc".to_string(),
        // ];
        // input = vec![
        //     "fs-end".to_string(),
        //     "he-DX".to_string(),
        //     "fs-he".to_string(),
        //     "start-DX".to_string(),
        //     "pj-DX".to_string(),
        //     "end-zg".to_string(),
        //     "zg-sl".to_string(),
        //     "zg-pj".to_string(),
        //     "pj-he".to_string(),
        //     "RW-he".to_string(),
        //     "fs-DX".to_string(),
        //     "pj-RW".to_string(),
        //     "zg-RW".to_string(),
        //     "start-pj".to_string(),
        //     "he-WI".to_string(),
        //     "zg-he".to_string(),
        //     "pj-fs".to_string(),
        //     "start-RW".to_string(),
        // ];
    }

    match part {
        "1" => {
            // let result = day12p1(&input);
            let result = day12p1_v2(&input);
            println!("Result: {}", result);
        }
        "2" => {
            let result = day12p2(&input);
            println!("Result: {}", result);
        }
        _ => (),
    }
}

// #[derive(Debug, Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
// struct Node<'a> {
//     name: &'a str,
//     can_multiple: bool,
//     can_use: bool,
// }

// impl<'a> Node<'a> {
//     /// Creates a new Node.
//     fn new(name: &'a str) -> Node {
//         let mut can_multiple = true;

//         if name != "start" || name != "end" {
//             can_multiple = !name.chars().nth(0).unwrap().is_ascii_lowercase();
//         }

//         Node {
//             name: name,
//             can_multiple: can_multiple,
//             can_use: true,
//         }
//     }
// }

fn day12p1_v2(input: &[String]) -> i32 {
    // println!("{:?}", input);
    let mut graph: HashMap<String, Vec<&str>> = HashMap::new();
    for item in input {
        let nodes = item.split("-").into_iter().collect::<Vec<&str>>();
        let n0 = nodes[0];
        let n1 = nodes[1];
        match graph.get_mut(n0) {
            Some(node_list) => {
                node_list.push(n1.clone());
            }
            None => {
                graph.insert(n0.to_string(), vec![n1.clone()]);
            }
        }

        match graph.get_mut(n1) {
            Some(node_list) => {
                node_list.push(n0.clone());
            }
            None => {
                graph.insert(n1.to_string(), vec![n0.clone()]);
            }
        }
    }
    graph.remove(&"end".to_string()).unwrap();
    // println!("{:?}", graph);

    let mut graph_path: Vec<String> = vec![];
    travel_node_v2(
        "start".to_string(),
        &mut graph_path,
        // &vec![],
        &vec!["start".to_string()],
        &vec!["start".to_string()],
        &graph,
    );

    println!("{:?}", graph_path);
    println!("{:?}", graph_path.len());
    graph_path.len() as i32
}

fn is_reusable(node: &str) -> bool {
    node.to_ascii_lowercase() != node
}

fn travel_node_v2(
    starter: String,
    graph_path: &mut Vec<String>,
    visited: &Vec<String>,
    block_list: &Vec<String>,
    graph: &HashMap<String, Vec<&str>>,
) {
    // println!("==============");
    // println!("starter {:?}", starter);
    // println!("graph_path {:?}", graph_path);
    // println!("visited {:?}", visited);
    // println!("block_list {:?}", block_list);
    // println!("graph {:?}", graph);
    // println!("==============");
    // let mut sub_node;
    let sub_node;

    // visited.push(starter.clone());
    match graph.get(&starter.clone()) {
        Some(x) => sub_node = x,
        None => return,
    }

    for (_idx, &elem) in sub_node.into_iter().enumerate() {
        let mut history = visited.clone();
        if elem == "end" {
            history.push(elem.to_string());
            let path = history.join(",");
            graph_path.push(path);
            continue;
        }
        if elem == "start" || block_list.contains(&elem.to_string()) {
            continue;
        }
        history.push(elem.to_string());
        // println!("{:?} {:?}", idx, elem);
        let mut b_list = block_list.clone();
        if !is_reusable(elem) {
            b_list.push(elem.to_string());
        }
        travel_node_v2(elem.to_string(), graph_path, &history, &b_list, &graph);
    }
}

fn day12p2(input: &[String]) -> i32 {
    let mut graph: HashMap<String, Vec<&str>> = HashMap::new();
    for item in input {
        let nodes = item.split("-").into_iter().collect::<Vec<&str>>();
        let n0 = nodes[0];
        let n1 = nodes[1];
        match graph.get_mut(n0) {
            Some(node_list) => {
                node_list.push(n1.clone());
            }
            None => {
                graph.insert(n0.to_string(), vec![n1.clone()]);
            }
        }

        match graph.get_mut(n1) {
            Some(node_list) => {
                node_list.push(n0.clone());
            }
            None => {
                graph.insert(n1.to_string(), vec![n0.clone()]);
            }
        }
    }
    graph.remove(&"end".to_string()).unwrap();

    let mut graph_path: Vec<String> = vec![];
    travel_node_part2(
        "start".to_string(),
        &mut graph_path,
        // &vec![],
        &vec!["start".to_string()],
        &vec!["start".to_string()],
        HashMap::from([]),
        false,
        &graph,
    );

    // println!("{:?}", graph_path);
    // println!("{:?}", graph_path.len());
    graph_path.len() as i32
}

fn travel_node_part2(
    starter: String,
    graph_path: &mut Vec<String>,
    visited: &Vec<String>,
    block_list: &Vec<String>,
    block_bucket: HashMap<String, i32>,
    block_trigger: bool,
    graph: &HashMap<String, Vec<&str>>,
) {
    let sub_node;

    match graph.get(&starter.clone()) {
        Some(x) => sub_node = x,
        None => return,
    }

    for &elem in sub_node.iter() {
        let mut history = visited.clone();
        if elem == "end" {
            history.push(elem.to_string());
            let path = history.join(",");
            graph_path.push(path);
            continue;
        }
        if elem == "start" || block_list.contains(&elem.to_string()) {
            continue;
        }

        history.push(elem.to_string());
        let mut b_list = block_list.clone();
        let mut b_bucket = block_bucket.clone();
        let mut trigger = false;
        if !is_reusable(elem) {
            match b_bucket.get(elem) {
                Some(_v) => {
                    trigger = true;
                    for idx in (0..history.len()).rev() {
                        if !is_reusable(&history[idx]) {
                            b_list.push(history[idx].to_string());
                        }
                    }
                }
                None => {
                    b_bucket.insert(elem.to_string(), 1);
                }
            }
            if trigger || block_trigger {
                b_list.push(elem.to_string());
            }
        }

        travel_node_part2(
            elem.to_string(),
            graph_path,
            &history,
            &b_list,
            b_bucket,
            trigger || block_trigger,
            &graph,
        );
    }
}

// fn is_reusable_part2(node: &str, bucket: HashSet<String>) -> bool {
//     node.to_ascii_lowercase() != node
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn day12p1v1() {
//         let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
//         let result = day12p1(&input);
//         let expect = 7;
//         assert_eq!(result, expect);
//     }
//     #[test]
//     fn day12p2v1() {
//         let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
//         let result = day12p2(&input);
//         let expect = 5;
//         assert_eq!(result, expect);
//     }
// }
