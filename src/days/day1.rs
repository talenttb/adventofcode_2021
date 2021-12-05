use crate::days::libs::read_lines;

pub fn solution(part: &str, ff: bool) {
    let mut input = Vec::new();
    if ff {
        if let Ok(lines) = read_lines("./src/days/day1input") {
            for line in lines {
                if let Ok(l) = line {
                    // println!("{}", l);
                    input.push(l.parse::<i32>().unwrap());
                }
            }
        }
    } else {
        input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    }

    match part {
        "1" => {
            let result = d1p1(&input);
            println!("Result: {}", result);
        }
        "2" => {
            let result = d1p2(&input);
            println!("Result: {}", result);
        }
        _ => (),
    }
}

fn d1p1(arr: &[i32]) -> i32 {
    let mut result = 0;
    let mut tmp = arr[0];
    for idx in 1..arr.len() {
        if tmp < arr[idx] {
            result += 1;
        }
        tmp = arr[idx];
    }
    return result;
}

fn d1p2(arr: &[i32]) -> i32 {
    // let mut result = 0;
    let mut input: Vec<i32> = Vec::new();
    // let mut tmp = arr[0];
    for idx in 2..arr.len() {
        input.push(arr[idx - 2] + arr[idx - 1] + arr[idx]);
    }
    // println!("{:?}", input);
    return d1p1(&input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d1p1v1() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let result = d1p1(&input);
        let expect = 7;
        assert_eq!(result, expect);
    }
    #[test]
    fn d1p2v1() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let result = d1p2(&input);
        let expect = 5;
        assert_eq!(result, expect);
    }
}
