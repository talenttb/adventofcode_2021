use crate::days::libs::read_lines;
use std::collections::HashMap;

pub fn solution(part: &str, ff: bool) {
    let mut input = Vec::new();
    if ff {
        if let Ok(lines) = read_lines("./src/days/day10input") {
            for line in lines {
                if let Ok(l) = line {
                    // println!("{}", l);
                    input.push(l);
                }
            }
        }
    } else {
        input = vec![
            "[({(<(())[]>[[{[]{<()<>>".to_string(),
            "[(()[<>])]({[<{<<[]>>(".to_string(),
            "{([(<{}[<>[]}>{[]{[(<()>".to_string(),
            "(((({<>}<{<{<>}{[]{[]{}".to_string(),
            "[[<[([]))<([[{}[[()]]]".to_string(),
            "[{[{({}]{}}([{[{{{}}([]".to_string(),
            "{<[[]]>}<{[{[{[]{()[[[]".to_string(),
            "[<(<(<(<{}))><([]([]()".to_string(),
            "<{([([[(<>()){}]>(<<{{".to_string(),
            "<{([{{}}[<[[[<>{}]]]>[]]".to_string(),
        ];
        // input = vec!["[(()[<>])]({[<{<<[]>>(".to_string()];
    }

    match part {
        "1" => {
            let result = day10p1(&input);
            println!("Result: {}", result);
        }
        "2" => {
            let result = day10p2(&input);
            println!("Result: {}", result);
        }
        _ => (),
    }
}

fn day10p1(input: &[String]) -> i32 {
    // println!("{:?}", input);
    let mut result = 0;
    // let pairs = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let reverse_pairs = HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);
    let scores = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    for item in input {
        // ( [ { < ) ] } >
        // let mut records = vec![0, 0, 0, 0, 0, 0, 0, 0];
        // println!("{:?}", col);
        let char_list: Vec<char> = item.chars().collect();
        let mut records = vec![1; item.len()];
        // println!("{:?}", records);
        'SEARCH: for (idx, ch) in item.chars().enumerate() {
            // println!("{} -> {:?}", idx, ch);
            match ch {
                // '(' => (records[0] += 1),
                // '[' => (records[1] += 1),
                // '{' => (records[2] += 1),
                // '>' => (records[3] += 1),
                // ')' => (),
                // ']' => (),
                // '}' => (),
                // '>' => (),
                // '(' | '[' | '{' | '<' => (),
                ')' | ']' | '}' | '>' => {
                    records[idx] = 0;
                    let mut i = idx - 1;
                    loop {
                        // if i < 0 {
                        //     break;
                        // }

                        if records[i] == 1 {
                            let expect = reverse_pairs.get(&ch).unwrap();
                            if expect == &char_list[i] {
                                records[i] = 0;
                                break;
                            } else {
                                // let my_expect = pairs.get(&char_list[i]).unwrap();
                                // println!(
                                //     "expect {}, but found {:?} instand. we are at {} => {}",
                                //     my_expect, ch, idx, char_list[idx],
                                // );
                                result += scores.get(&ch).unwrap();
                                break 'SEARCH;
                            }
                        }
                        i -= 1;
                    }
                    // println!("{:?} {}", char_list[idx - 1], char_list[idx]);
                    // println!("{:?} {}", idx, char_list.nth(idx - 1).unwrap());
                }
                _ => (),
            }
        }
    }

    result
}

fn day10p2(input: &[String]) -> u64 {
    let mut result: Vec<_> = vec![];
    let pairs = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let reverse_pairs = HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);
    let scores = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);
    'SEARCH: for item in input {
        let char_list: Vec<char> = item.chars().collect();
        let mut records = vec![1; item.len()];
        for (idx, ch) in item.chars().enumerate() {
            match ch {
                ')' | ']' | '}' | '>' => {
                    records[idx] = 0;
                    let mut i = idx - 1;
                    loop {
                        if records[i] == 1 {
                            let expect = reverse_pairs.get(&ch).unwrap();
                            if expect == &char_list[i] {
                                records[i] = 0;
                                break;
                            } else {
                                continue 'SEARCH;
                            }
                        }
                        if i > 0 {
                            i -= 1;
                        } else {
                            break;
                        }
                    }
                }
                _ => (),
            }
        }

        let mut sum: u64 = 0;
        for idx in (0..records.len()).rev() {
            if records[idx] == 1 {
                let p = pairs.get(&char_list[idx]).unwrap();
                let s = scores.get(&p).unwrap();
                sum = sum * 5 + s;
            }
        }
        if sum > 0 {
            result.push(sum);
        }
    }
    result.sort();
    result[(result.len() / 2)]
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn day10p1v1() {
//         let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
//         let result = day10p1(&input);
//         let expect = 7;
//         assert_eq!(result, expect);
//     }
//     #[test]
//     fn day10p2v1() {
//         let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
//         let result = day10p2(&input);
//         let expect = 5;
//         assert_eq!(result, expect);
//     }
// }
