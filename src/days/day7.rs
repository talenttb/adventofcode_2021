use crate::days::libs::read_lines;
use std::collections::HashMap;

pub fn solution(part: &str, ff: bool) {
    let mut input = Vec::new();
    if ff {
        if let Ok(lines) = read_lines("./src/days/day7input") {
            for line in lines {
                if let Ok(l) = line {
                    // println!("{}", l);
                    // input.push(l.parse::<i32>().unwrap());
                    for v in l.split(",") {
                        input.push(v.parse::<i32>().unwrap());
                    }
                }
            }
        }
    } else {
        input = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
    }

    match part {
        "1" => {
            let result = day7p1(&input);
            println!("Result: {}", result);
        }
        "2" => {
            let result = day7p2(&input);
            println!("Result: {}", result);
        }
        _ => (),
    }
}

fn day7p1(input: &[i32]) -> i32 {
    // println!("{:?}", input);
    let mut best_fuel = i32::MAX;
    for i in 0..input.len() {
        let align = input[i];
        let mut fuel = 0;
        for item in input.iter() {
            fuel += (item - align).abs();
            if fuel >= best_fuel {
                break;
            }
        }
        if fuel < best_fuel {
            best_fuel = fuel;
        }
    }
    // println!("{}", best_fuel);
    best_fuel
}

fn day7p2(input: &[i32]) -> i32 {
    let mut best_fuel = i32::MAX;

    let mut distance_fuel: HashMap<i32, i32> = HashMap::new();
    for pos in 0..input.len() {
        // for pos in 5..6 {
        // let align = input[pos];
        let mut fuel = 0;
        for item in input.iter() {
            let distance: i32 = (item - (pos as i32)).abs();
            if !distance_fuel.contains_key(&distance) {
                let mut tmp: i32 = 0;
                for i in 1..=distance {
                    tmp += i;
                }
                // distance_fuel[&distance] = tmp;
                distance_fuel.insert(distance, tmp);
            }
            let cost_fuel = distance_fuel.get(&distance).unwrap();
            // println!("from {} to pos {}: {} fuel", item, pos, cost_fuel);
            fuel += cost_fuel;
            if fuel >= best_fuel {
                break;
            }
        }
        // println!("cache: {:?}", distance_fuel);

        println!("alignment position {}, result {}", pos, fuel);
        if fuel < best_fuel {
            best_fuel = fuel;
        }
    }
    // println!("{}", best_fuel);
    best_fuel
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn day7p1v1() {
//         let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
//         let result = day7p1(&input);
//         let expect = 7;
//         assert_eq!(result, expect);
//     }
//     #[test]
//     fn day7p2v1() {
//         let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
//         let result = day7p2(&input);
//         let expect = 5;
//         assert_eq!(result, expect);
//     }
// }
