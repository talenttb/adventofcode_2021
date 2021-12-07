use crate::days::libs::read_lines;

pub fn solution(part: &str, ff: bool) {
    let mut input = Vec::new();
    if ff {
        if let Ok(lines) = read_lines("./src/days/dayXinput") {
            for line in lines {
                if let Ok(l) = line {
                    println!("{}", l);
                    // input.push(l.parse::<i32>().unwrap());
                    // for v in l.split(",") {
                    //     input.push(v.parse::<i32>().unwrap());
                    // }
                }
            }
        }
    } else {
        input = vec![];
    }

    match part {
        "1" => {
            let result = dayXp1(&input);
            println!("Result: {}", result);
        }
        "2" => {
            let result = dayXp2(&input);
            println!("Result: {}", result);
        }
        _ => (),
    }
}

fn dayXp1(input: &[i32]) -> i32 {
    println!("{:?}", input);
    0
}

fn dayXp2(input: &[i32]) -> i32 {
    println!("{:?}", input);
    0
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn dayXp1v1() {
//         let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
//         let result = dayXp1(&input);
//         let expect = 7;
//         assert_eq!(result, expect);
//     }
//     #[test]
//     fn dayXp2v1() {
//         let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
//         let result = dayXp2(&input);
//         let expect = 5;
//         assert_eq!(result, expect);
//     }
// }
