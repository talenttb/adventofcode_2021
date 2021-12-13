use crate::days::libs::read_lines;

pub fn solution(part: &str, ff: bool) {
    let mut input = Vec::new();
    if ff {
        if let Ok(lines) = read_lines("./src/days/day11input") {
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
            "5483143223".to_string(),
            "2745854711".to_string(),
            "5264556173".to_string(),
            "6141336146".to_string(),
            "6357385478".to_string(),
            "4167524645".to_string(),
            "2176841721".to_string(),
            "6882881134".to_string(),
            "4846848554".to_string(),
            "5283751526".to_string(),
        ];
        // input = vec![
        //     "11111".to_string(),
        //     "19991".to_string(),
        //     "19191".to_string(),
        //     "19991".to_string(),
        //     "11111".to_string(),
        // ];
    }

    match part {
        "1" => {
            let result = day11p1(&input);
            println!("Result: {}", result);
        }
        "2" => {
            let result = day11p2(&input);
            println!("Result: {}", result);
        }
        _ => (),
    }
}

fn day11p1(input: &[String]) -> i32 {
    // println!("{:?}", input);

    let mut board = input
        .iter()
        .map(|item| {
            item.split("")
                .map(|x| x.parse::<i32>())
                // .inspect(|x| println!("{:?}", x))
                .filter_map(Result::ok)
                .collect::<Vec<i32>>()
        })
        // .inspect(|x| println!("x2: {:?}", x))
        .collect::<Vec<Vec<i32>>>();
    println!("{:?}", board);

    let mut freshes = 0;
    for step in 1..=100 {
        // println!("{:?}", step - 1 > 0);
        // if step - 1 > 0 {
        //     println!("TEST {:?}", step);
        // }
        for row in &mut board {
            for item in row.iter_mut() {
                *item += 1;
            }
        }
        check_flash(&mut board);
        for row in &mut board {
            for item in row.iter_mut() {
                if *item == 0 {
                    freshes += 1;
                }
            }
        }
        println!("After step {:?}\n{:?}", step, board);
    }
    // println!("total freshes: {:?}", freshes);
    freshes
}

fn check_flash(board: &mut Vec<Vec<i32>>) {
    let mut stop = true;
    let (row_size, col_size) = (board.len(), board[0].len());

    while stop {
        stop = false;
        // println!("{:?}", board);
        for i in 0..board.len() {
            for j in 0..board[i].len() {
                let value = board[i][j];
                // stop |= value > 9;
                if value > 9 {
                    stop |= true;
                    board[i][j] = 0;
                } else {
                    continue;
                }
                // ↖
                if i > 0 && j > 0 && board[i - 1][j - 1] != 0 {
                    board[i - 1][j - 1] += 1;
                    stop |= board[i - 1][j - 1] > 9;
                }
                // ↑
                if i > 0 && board[i - 1][j] != 0 {
                    board[i - 1][j] += 1;
                    stop |= board[i - 1][j] > 9;
                }
                // ↗
                if i > 0 && j + 1 < col_size && board[i - 1][j + 1] != 0 {
                    board[i - 1][j + 1] += 1;
                    stop |= board[i - 1][j + 1] > 9;
                }
                // ←
                if j > 0 && board[i][j - 1] != 0 {
                    board[i][j - 1] += 1;
                    stop |= board[i][j - 1] > 9;
                }
                // →
                if j + 1 < col_size && board[i][j + 1] != 0 {
                    board[i][j + 1] += 1;
                    stop |= board[i][j + 1] > 9;
                }
                // ↙
                if i + 1 < row_size && j > 0 && board[i + 1][j - 1] != 0 {
                    board[i + 1][j - 1] += 1;
                    stop |= board[i + 1][j - 1] > 9;
                }
                // ↓
                if i + 1 < row_size && board[i + 1][j] != 0 {
                    board[i + 1][j] += 1;
                    stop |= board[i + 1][j] > 9;
                }
                // ↘
                if i + 1 < row_size && j + 1 < col_size && board[i + 1][j + 1] != 0 {
                    board[i + 1][j + 1] += 1;
                    stop |= board[i + 1][j + 1] > 9;
                }
                // println!("{:?}", board);
            }
        }
    }
}

fn day11p2(input: &[String]) -> i32 {
    let mut board = input
        .iter()
        .map(|item| {
            item.split("")
                .filter_map(|x| x.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut step = 0;
    loop {
        step += 1;
        for row in &mut board {
            for item in row.iter_mut() {
                *item += 1;
            }
        }
        check_flash(&mut board);
        // println!("After step {:?}\n{:?}", step, board);

        let freshes: usize = board
            .iter()
            .map(|item| item.iter().filter(|&&x| x == 0).collect::<Vec<_>>().len())
            .collect::<Vec<usize>>()
            .iter()
            .sum();
        if freshes == 100 {
            break;
        }
    }
    step
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn day11p1v1() {
//         let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
//         let result = day11p1(&input);
//         let expect = 7;
//         assert_eq!(result, expect);
//     }
//     #[test]
//     fn day11p2v1() {
//         let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
//         let result = day11p2(&input);
//         let expect = 5;
//         assert_eq!(result, expect);
//     }
// }
