use crate::days::libs::read_lines;

pub fn solution(part: &str, ff: bool) {
    let mut input: Vec<String> = Vec::new();
    if ff {
        if let Ok(lines) = read_lines("./src/days/day2input") {
            for line in lines {
                if let Ok(l) = line {
                    input.push(l);
                    // input.push(l.parse::<i32>().unwrap());
                }
            }
        }
    } else {
        input = vec![
            "forward 5".to_string(),
            "down 5".to_string(),
            "forward 8".to_string(),
            "up 3".to_string(),
            "down 8".to_string(),
            "forward 2".to_string(),
        ];
    }

    match part {
        "1" => {
            let result = d2p1(&input);
            println!("Result: {}", result);
        }
        "2" => {
            let result = d2p2(&input);
            println!("Result: {}", result);
        }
        _ => (),
    }
}

fn d2p1(arr: &[String]) -> i32 {
    let (mut depth, mut position): (i32, i32) = (0, 0);
    for item in arr {
        let s: Vec<&str> = item.split(" ").collect();
        // println!("{:?}", s);
        let n = s[1].parse::<i32>().unwrap();
        match s[0] {
            "forward" => position += n,
            "up" => depth -= n,
            "down" => depth += n,
            _ => (),
        }
    }
    return depth * position;
}

fn d2p2(arr: &[String]) -> i32 {
    let (mut depth, mut position, mut aim) = (0, 0, 0);
    for item in arr {
        let s: Vec<&str> = item.split(" ").collect();
        // println!("{:?}", s);
        let n = s[1].parse::<i32>().unwrap();
        match s[0] {
            "forward" => {
                position += n;
                depth += n * aim;
            }
            "up" => aim -= n,
            "down" => aim += n,
            _ => (),
        }
        // println!("position: {}, depth: {}, aim: {}", position, depth, aim);
    }
    return depth * position;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d2p1v1() {
        let input = vec![
            "forward 5".to_string(),
            "down 5".to_string(),
            "forward 8".to_string(),
            "up 3".to_string(),
            "down 8".to_string(),
            "forward 2".to_string(),
        ];
        let result = d2p1(&input);
        let expect = 150;
        assert_eq!(result, expect);
    }

    #[test]
    fn d2p2v1() {
        let input = vec![
            "forward 5".to_string(),
            "down 5".to_string(),
            "forward 8".to_string(),
            "up 3".to_string(),
            "down 8".to_string(),
            "forward 2".to_string(),
        ];
        let result = d2p2(&input);
        let expect = 900;
        assert_eq!(result, expect);
    }
}
