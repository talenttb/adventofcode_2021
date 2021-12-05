use crate::days::libs::read_lines;

pub fn solution(part: &str, ff: bool) {
    let mut input: Vec<String> = Vec::new();
    if ff {
        if let Ok(lines) = read_lines("./src/days/day3input") {
            for line in lines {
                if let Ok(l) = line {
                    // println!("{}", l);
                    input.push(l);
                    // input.push(l.parse::<i32>().unwrap());
                }
            }
        }
    } else {
        input = vec![
            "00100".to_string(),
            "11110".to_string(),
            "10110".to_string(),
            "10111".to_string(),
            "10101".to_string(),
            "01111".to_string(),
            "00111".to_string(),
            "11100".to_string(),
            "10000".to_string(),
            "11001".to_string(),
            "00010".to_string(),
            "01010".to_string(),
        ];
    }

    match part {
        "1" => {
            let result = d3p1(&input);
            println!("Result: {}", result);
        }
        "2" => {
            let result = d3p2(&input);
            println!("Result: {}", result);
        }
        _ => (),
    }
}

fn d3p1(arr: &[String]) -> i32 {
    let l = arr[0].len();
    let mut register = vec![0; l];
    for item in arr {
        for (idx, ch) in item.chars().enumerate() {
            match ch {
                '0' => register[idx] -= 1,
                '1' => register[idx] += 1,
                _ => (),
            }
        }
    }
    let mut gamma = String::from("");
    let mut epsilon = String::from("");
    for elem in register {
        if elem > 0 {
            gamma.push_str("1");
            epsilon.push_str("0");
        } else {
            gamma.push_str("0");
            epsilon.push_str("1");
        }
    }
    let gamma_int = isize::from_str_radix(&gamma, 2).unwrap();
    let epsilon_int = isize::from_str_radix(&epsilon, 2).unwrap();

    return (gamma_int * epsilon_int) as i32;
}

fn d3p2(arr: &Vec<String>) -> i32 {
    let l = arr[0].len();

    let mut ox_running: Vec<String> = arr.clone();
    let mut ox0: Vec<String> = Vec::new();
    let mut ox1: Vec<String> = Vec::new();

    for i in 0..l {
        if ox_running.len() > 1 {
            for item in ox_running.iter() {
                // println!("{:?}", item.chars().nth(i).unwrap());
                match item.chars().nth(i).unwrap() {
                    '0' => ox0.push(item.to_string()),
                    '1' => ox1.push(item.to_string()),
                    _ => (),
                }
            }
            // println!("{:?}", ox0);
            // println!("{:?}", ox1);
            ox_running.clear();
            if ox0.len() > ox1.len() {
                ox_running = ox0.clone();
            } else {
                ox_running = ox1.clone();
            }
            ox0.clear();
            ox1.clear();
        }
    }
    // println!("{:?}", ox_running);
    let ox_int = isize::from_str_radix(&ox_running[0], 2).unwrap();
    // println!("{:?}", ox_int);
    let mut co2_running: Vec<String> = arr.clone();
    let mut co20: Vec<String> = Vec::new();
    let mut co21: Vec<String> = Vec::new();

    for i in 0..l {
        if co2_running.len() > 1 {
            for item in co2_running.iter() {
                // println!("{:?}", item.chars().nth(i).unwrap());
                match item.chars().nth(i).unwrap() {
                    '0' => co20.push(item.to_string()),
                    '1' => co21.push(item.to_string()),
                    _ => (),
                }
            }
            // println!("{:?}", co20);
            // println!("{:?}", co21);
            co2_running.clear();
            if co20.len() <= co21.len() {
                co2_running = co20.clone();
            } else {
                co2_running = co21.clone();
            }
            co20.clear();
            co21.clear();
        }
    }
    // println!("{:?}", co2_running);
    let co2_int = isize::from_str_radix(&co2_running[0], 2).unwrap();
    // println!("{:?}", co2_int);

    (ox_int * co2_int) as i32
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn d3p1v1() {
    //     let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    //     let result = d3p1(&input);
    //     let expect = 7;
    //     assert_eq!(result, expect);
    // }
    // #[test]
    // fn d3p2v1() {
    //     let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    //     let result = d3p2(&input);
    //     let expect = 5;
    //     assert_eq!(result, expect);
    // }
}
