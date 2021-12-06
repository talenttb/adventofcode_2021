use crate::days::libs::read_lines;

pub fn solution(part: &str, ff: bool) {
    let mut input = Vec::new();
    if ff {
        if let Ok(lines) = read_lines("./src/days/day6input") {
            for line in lines {
                if let Ok(l) = line {
                    // println!("{}", l);
                    // input.push(v.parse::<i32>().unwrap());
                    for v in l.split(",") {
                        input.push(v.parse::<i32>().unwrap());
                    }
                }
            }
        }
    } else {
        // 3,4,3,1,2
        // 3,3=>2308
        // 3=>1154
        // 4=>1034
        // 1=>1401
        // 2=>1191
        // let (a, b, c, d, e): (u64, u64, u64, u64, u64) = (1154, 1154, 1034, 1401, 1191);
        // println!("{}", a * b * c * d * e);
        input = vec![3, 4, 3, 1, 2];
    }

    match part {
        "1" => {
            let result = day6p1(&input);
            // let result = day6p1(input.as_mut_slice());
            println!("Result: {}", result);
        }
        "2" => {
            // let result = day6p2(&input);
            let result = day6p2v2(&input);
            println!("Result: {}", result);
        }
        _ => (),
    }
}

fn day6p1(fishes: &[i32]) -> i32 {
    // println!("{:?}", fishes);
    let mut fish_pool = fishes.to_vec();
    for _i in 0..80 {
        println!("day {:0>2}({}): {:?}", _i, fish_pool.len(), fish_pool);

        let mut adding = 0;

        for fish in fish_pool.iter_mut() {
            match fish {
                0 => {
                    *fish = 6;
                    adding += 1
                }
                _ => *fish -= 1,
            }
        }

        for _ in 0..adding {
            fish_pool.push(8);
        }
    }
    // let mut result = 0;
    // for i in fish_pool {
    //     result += i;
    // }
    // result
    fish_pool.len() as i32
}

#[allow(dead_code)]
fn day6p2(fishes: &[i32]) -> u64 {
    // println!("{:?}", fishes);
    let mut fish_pool = fishes.to_vec();
    for _i in 0..256 {
        // println!("day {: >2}: {:?}", _i, fish_pool.len());

        let mut adding = 0;

        for fish in fish_pool.iter_mut() {
            match fish {
                0 => {
                    *fish = 6;
                    adding += 1
                }
                _ => *fish -= 1,
            }
        }

        for _ in 0..adding {
            fish_pool.push(8);
        }
    }
    // let mut result = 0;
    // for i in fish_pool {
    //     result += i;
    // }
    // result
    fish_pool.len() as u64
}

fn day6p2v2(fishes: &[i32]) -> u64 {
    // day6p2v2
    // let ages_list = aoc2021::comma_separated_to_usize_vec(data);
    // let data = "3,4,3,1,2";
    // let ages_list: Vec<usize> = data.trim().split(",").map(|s| s.parse().unwrap()).collect();
    let mut ages = [0; 9];
    for age in fishes {
        ages[*age as usize] += 1;
    }

    // println!("{} {:?}", -1, ages);
    for _day in 0..256 {
        // Rotate the array so every timer decrements, and the re-spawning ones
        // wrap to index 8. Also, the amount that wrapped to 8 should add to index 6.
        ages.rotate_left(1);
        ages[6] += ages[8];
        // println!("{} {:?}", _day, ages);
    }
    ages.iter().sum()
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn day6p1v1() {
//         let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
//         let result = day6p1(&input);
//         let expect = 7;
//         assert_eq!(result, expect);
//     }
//     #[test]
//     fn day6p2v1() {
//         let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
//         let result = day6p2(&input);
//         let expect = 5;
//         assert_eq!(result, expect);
//     }
// }
