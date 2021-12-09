use crate::days::libs::read_lines;

pub fn solution(part: &str, ff: bool) {
    let mut input = Vec::new();
    if ff {
        if let Ok(lines) = read_lines("./src/days/day8input") {
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
        input = vec![
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe".to_string(),
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc".to_string(),
            "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg".to_string(),
            "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb".to_string(),
            "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea".to_string(),
            "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb".to_string(),
            "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe".to_string(),
            "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef".to_string(),
            "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb".to_string(),
            "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce".to_string(),
        ];
    }

    match part {
        "1" => {
            let result = day8p1(&input);
            println!("Result: {}", result);
        }
        "2" => {
            let result = day8p2(&input);
            println!("Result: {}", result);
        }
        _ => (),
    }
}

fn day8p1(input: &[String]) -> i32 {
    // println!("{:?}", input);
    // digit
    let mut uni_digit_count = 0;
    let uni_range = vec![2, 4, 3, 7];
    for item in input {
        let patters: Vec<&str> = item.split("|").collect();
        for digit in patters[1].split(" ") {
            if uni_range.contains(&digit.len()) {
                uni_digit_count += 1;
            }
        }
    }

    uni_digit_count
}

fn day8p2(input: &[String]) -> i32 {
    // println!("{:?}", input);
    use std::collections::HashMap;
    // use std::collections::{HashMap, HashSet};
    // let mut digits_7s_dsp = vec![vec!["c"], "f"];
    let mut digits_7s_dsp: HashMap<Vec<char>, i32> = HashMap::new();
    digits_7s_dsp.insert(vec!['a', 'b', 'c', 'e', 'f', 'g'], 0);
    digits_7s_dsp.insert(vec!['c', 'f'], 1);
    digits_7s_dsp.insert(vec!['a', 'c', 'd', 'e', 'g'], 2);
    digits_7s_dsp.insert(vec!['a', 'c', 'd', 'f', 'g'], 3);
    digits_7s_dsp.insert(vec!['b', 'c', 'd', 'f'], 4);
    digits_7s_dsp.insert(vec!['a', 'b', 'd', 'f', 'g'], 5);
    digits_7s_dsp.insert(vec!['a', 'b', 'd', 'e', 'f', 'g'], 6);
    digits_7s_dsp.insert(vec!['a', 'c', 'f'], 7);
    digits_7s_dsp.insert(vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'], 8);
    digits_7s_dsp.insert(vec!['a', 'b', 'c', 'd', 'f', 'g'], 9);
    // println!("{:?}", digits_7s_dsp);
    // let test = vec!["a", "b", "c", "e", "f", "g"];
    // let test = vec!["a", "b", "c", "e", "f", "g"];
    // let cost_fuel = digits_7s_dsp.contains_key(test).unwrap();
    // println!("{:?}=>{}", test, cost_fuel);
    // signal
    let mut result = 0;
    for item in input {
        let mut mapping: HashMap<char, char> = HashMap::new();
        // println!("{:?}", item);
        let pattern = item.split("|").collect::<Vec<&str>>().join("");

        let digits: Vec<&str> = pattern.split_whitespace().collect();

        let mut tmp4 = digits
            .iter()
            .filter(|item| item.len() == 4)
            .map(|item| *item)
            .collect::<Vec<&str>>();
        tmp4.sort();
        tmp4.dedup();
        let digit_4 = tmp4[0].chars().collect::<Vec<char>>();

        //step1 use 1/7 to will find "a"
        let mut step1 = digits
            .iter()
            .filter(|item| vec![2, 3].contains(&item.len()))
            .map(|item| {
                let mut t = item.chars().collect::<Vec<char>>();
                t.sort();
                let rtn = t
                    .iter()
                    .fold("".to_string(), |acc, &x| format!("{}{}", acc, x));
                rtn
            })
            .collect::<Vec<String>>();
        step1.sort();
        step1.dedup();
        assert_eq!(step1.len(), 2);
        let digit_1 = step1.iter().filter(|x| x.len() == 2).collect::<Vec<_>>()[0]
            .chars()
            .collect::<Vec<char>>();
        let mut digit_7 = step1.iter().filter(|x| x.len() == 3).collect::<Vec<_>>()[0]
            .chars()
            .collect::<Vec<char>>();
        // println!("{:?}", digit_7);
        digit_7.retain(|&x| !digit_1.contains(&x));
        mapping.insert(digit_7[0], 'a');

        //step2 use 2/3/5 to will find "b/c/e/f"
        let mut step2 = digits
            .iter()
            .filter(|item| item.len() == (5 as usize))
            .map(|item| {
                let mut t = item.chars().collect::<Vec<char>>();
                t.sort();
                let rtn = t
                    .iter()
                    .fold("".to_string(), |acc, &x| format!("{}{}", acc, x));
                rtn
            })
            .collect::<Vec<String>>();
        step2.sort();
        step2.dedup();

        let mut three_bars = step2[0].chars().collect::<Vec<char>>();
        let digit_235_v2 = step2[1].chars().collect::<Vec<char>>();
        let digit_235_v3 = step2[2].chars().collect::<Vec<char>>();
        three_bars.retain(|x| digit_235_v2.contains(&x) && digit_235_v3.contains(&x));
        // println!("3 bars {:?}", three_bars);

        for item in step2.clone() {
            let mut check_b = item.chars().collect::<Vec<char>>();
            check_b.retain(|x| !(digit_1.contains(&x) || three_bars.contains(&x)));
            check_b.retain(|x| digit_4.contains(&x));
            if check_b.len() == 1 {
                mapping.insert(check_b[0], 'b');
                break;
            }
        }

        for item in step2.clone() {
            let mut check_b = item.chars().collect::<Vec<char>>();
            check_b.retain(|x| !(digit_1.contains(&x) || three_bars.contains(&x)));
            check_b.retain(|&x| !mapping.contains_key(&x));
            if check_b.len() == 1 {
                mapping.insert(check_b[0], 'e');
                break;
            }
        }

        for item in step2.clone() {
            let mut check_b = item.chars().collect::<Vec<char>>();
            check_b.retain(|x| !(three_bars.contains(&x)));
            for (idx, elem) in check_b.iter().enumerate() {
                match mapping.get(&elem) {
                    Some(x) => match x {
                        'b' => match idx {
                            0 => {
                                mapping.insert(check_b[1], 'f');
                            }
                            1 => {
                                mapping.insert(check_b[0], 'f');
                            }
                            _ => (),
                        },
                        'e' => match idx {
                            0 => {
                                mapping.insert(check_b[1], 'c');
                            }
                            1 => {
                                mapping.insert(check_b[0], 'c');
                            }
                            _ => (),
                        },
                        _ => (),
                    },
                    None => (),
                }
            }
        }

        for elem in digit_4.clone() {
            if !mapping.contains_key(&elem) {
                mapping.insert(elem, 'd');
            }
        }

        for elem in three_bars.clone() {
            if !mapping.contains_key(&elem) {
                mapping.insert(elem, 'g');
            }
        }

        // println!("mapping: {:?}", mapping);
        // println!("mapping: {:?}", digits[..digits.len() - 4]);

        let mut decrypt_str: String = "".to_owned();
        for i in 0..4 {
            let d = digits[digits.len() - 4 + i];

            let mut decrypt_digit_str: String = "".to_owned();
            // let another_owned_string: String = "world".to_owned();
            for elem in d.chars() {
                let n = mapping.get(&elem).unwrap();
                decrypt_digit_str.push_str(&n.to_string());
            }
            let mut decrypt_char = decrypt_digit_str.chars().collect::<Vec<char>>();
            decrypt_char.sort();
            let r = digits_7s_dsp.get(&decrypt_char).unwrap();
            // println!("{:?}=>{}", test, cost_fuel);
            decrypt_str.push_str(&r.to_string());
            // println!("{:?} = {:?} = {:?}", d, decrypt_digit_str, r);
        }
        // println!("{:?} => {:?}", item, decrypt_str);
        // let my_int = my_string.parse::<i32>().unwrap();
        result += decrypt_str.parse::<i32>().unwrap();
    }
    result
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn day8p1v1() {
//         let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
//         let result = day8p1(&input);
//         let expect = 7;
//         assert_eq!(result, expect);
//     }
//     #[test]
//     fn day8p2v1() {
//         let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
//         let result = day8p2(&input);
//         let expect = 5;
//         assert_eq!(result, expect);
//     }
// }
