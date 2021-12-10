use crate::days::libs::read_lines;
use std::collections::HashMap;

pub fn solution(part: &str, ff: bool) {
    let mut input = vec![];
    if ff {
        if let Ok(lines) = read_lines("./src/days/day9input") {
            for line in lines {
                if let Ok(l) = line {
                    // println!("{}", l);
                    // input.push(l.parse::<i32>().unwrap());
                    let mut t = vec![];
                    for v in l.split("") {
                        if v != "" {
                            t.push(v.parse::<i32>().unwrap());
                        }
                    }
                    if t.len() > 0 {
                        input.push(t);
                    }
                }
            }
        }
    } else {
        input = vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ];
    }

    match part {
        "1" => {
            let result = day9p1(input);
            println!("Result: {}", result);
        }
        "2" => {
            let result = day9p2(input);
            println!("Result: {}", result);
        }
        _ => (),
    }
}

fn day9p1(input: Vec<Vec<i32>>) -> i32 {
    // println!("{:?}", input);

    let mut result = 0;
    for (i, row) in input.iter().enumerate() {
        // println!("check {:?}", row);
        for (j, &col) in row.iter().enumerate() {
            //up
            if i > 0 && col >= input[i - 1][j] {
                continue;
            }
            //down
            if i < input.len() - 1 && col >= input[i + 1][j] {
                continue;
            }
            //left
            if j > 0 && col >= input[i][j - 1] {
                continue;
            }
            //right
            if j < input[i].len() - 1 && col >= input[i][j + 1] {
                continue;
            }
            // println!("({},{})={:?}", i, j, col);
            result += col + 1;
        }
    }
    result
}

#[derive(Debug, Copy, Clone)]
struct Pos(usize, usize);

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Eq for Pos {}
use std::hash::{Hash, Hasher};
impl Hash for Pos {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
        self.1.hash(state);
    }
}

fn day9p2(input: Vec<Vec<i32>>) -> i32 {
    let mut m: HashMap<Pos, Pos> = HashMap::new();

    for (i, row) in input.iter().enumerate() {
        for (j, &col) in row.iter().enumerate() {
            let key = &Pos(i, j);
            if col != 9 && !m.contains_key(key) {
                recorded_map(input.clone(), i, j, key, &mut m);
            }
        }
    }

    let mut m_counter: HashMap<Pos, i32> = HashMap::new();
    for v in m.values() {
        *m_counter.entry(*v).or_insert(0) += 1;
    }

    let mut counter_list = vec![];
    for v in m_counter.values() {
        counter_list.push(v);
    }
    counter_list.sort_by(|a, b| b.cmp(a));
    counter_list[0] * counter_list[1] * counter_list[2]
}

fn recorded_map(input: Vec<Vec<i32>>, i: usize, j: usize, hash: &Pos, m: &mut HashMap<Pos, Pos>) {
    let v = input[i][j];
    let key = &Pos(i, j);
    if v == 9 || m.contains_key(key) {
        return;
    }
    m.insert(*key, *hash);

    //up
    if i > 0 && 9 != input[i - 1][j] {
        recorded_map(input.clone(), i - 1, j, &hash, m);
    }
    //down
    if i < input.len() - 1 && 9 != input[i + 1][j] {
        recorded_map(input.clone(), i + 1, j, &hash, m);
    }
    //left
    if j > 0 && 9 != input[i][j - 1] {
        recorded_map(input.clone(), i, j - 1, &hash, m);
    }
    //right
    if j < input[i].len() - 1 && 9 != input[i][j + 1] {
        recorded_map(input.clone(), i, j + 1, &hash, m);
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn day9p1v1() {
//         let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
//         let result = day9p1(&input);
//         let expect = 7;
//         assert_eq!(result, expect);
//     }
//     #[test]
//     fn day9p2v1() {
//         let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
//         let result = day9p2(&input);
//         let expect = 5;
//         assert_eq!(result, expect);
//     }
// }
