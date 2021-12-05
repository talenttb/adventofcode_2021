use crate::days::libs::read_lines;

pub fn solution(part: &str, ff: bool) {
    let mut input = Vec::new();
    let mut boards = vec![vec![vec![]]];
    if ff {
        let mut header = false;
        if let Ok(lines) = read_lines("./src/days/day4input") {
            let mut board = Vec::new();
            let mut row = Vec::new();
            for line in lines {
                if let Ok(l) = line {
                    // println!("{}", l);
                    if !header {
                        header = true;
                        for v in l.split(",") {
                            input.push(v.parse::<i32>().unwrap());
                        }
                        continue;
                    }

                    if l != "" {
                        for v in l.split(" ") {
                            if v != "" {
                                row.push(v.parse::<i32>().unwrap());
                            }
                        }

                        board.push(row.clone());
                        row.clear();

                        if board.len() == 5 {
                            boards.push(board.clone());
                            board.clear();
                        }
                    }
                }
            }
        }
    } else {
        input = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];
        boards = vec![
            vec![
                vec![22, 13, 17, 11, 0],
                vec![8, 2, 23, 4, 24],
                vec![21, 9, 14, 16, 7],
                vec![6, 10, 3, 18, 5],
                vec![1, 12, 20, 15, 19],
            ],
            vec![
                vec![3, 15, 0, 2, 22],
                vec![9, 18, 13, 17, 5],
                vec![19, 8, 7, 25, 23],
                vec![20, 11, 10, 24, 4],
                vec![14, 21, 16, 12, 6],
            ],
            vec![
                vec![14, 21, 17, 24, 4],
                vec![10, 16, 15, 9, 19],
                vec![18, 8, 23, 26, 20],
                vec![22, 11, 13, 6, 5],
                vec![2, 0, 12, 3, 7],
            ],
        ];
    }

    match part {
        "1" => {
            let result = day4p1(&input, boards);
            println!("Result: {}", result);
        }
        "2" => {
            let result = day4p2(&input, boards);
            println!("Result: {}", result);
        }
        _ => (),
    }
}

fn day4p1(lottery_nums: &[i32], boards: Vec<Vec<Vec<i32>>>) -> i32 {
    let board_size = boards.len();

    let mut boards_result = Vec::new();
    let board_result = vec![vec![0; 5], vec![0; 5], vec![0], vec![0]];
    for _ in 0..board_size {
        boards_result.push(board_result.clone());
    }

    let mut winner_board = board_size;
    let mut winner_step = lottery_nums.len();

    'CHECKING: for idx in 0..lottery_nums.len() {
        winner_step = idx;
        for (board_idx, board) in boards.iter().enumerate() {
            for (i, row) in board.iter().enumerate() {
                for (j, &col) in row.iter().enumerate() {
                    if col == lottery_nums[idx] {
                        //row
                        boards_result[board_idx][0][i] += 1;
                        if boards_result[board_idx][0][i] == 5 {
                            winner_board = board_idx;
                            break 'CHECKING;
                        }
                        //col
                        boards_result[board_idx][1][j] += 1;
                        if boards_result[board_idx][1][j] == 5 {
                            winner_board = board_idx;
                            break 'CHECKING;
                        }
                        //backslash
                        // boards_result[board_idx][2][0] += if i == j { 1 } else { 0 };
                        // if boards_result[board_idx][2][0] == 5 {
                        //     winner = board_idx;
                        //     break 'CHECKING;
                        // }
                        //forwardslash
                        // boards_result[board_idx][3][0] += if i + j == 4 { 1 } else { 0 };
                        // if boards_result[board_idx][3][0] == 5 {
                        //     winner = board_idx;
                        //     break 'CHECKING;
                        // }
                    }
                }
            }
        }
    }

    // println!(
    //     "FOUND! Winner: {} with steps: {}, final lottory: {}",
    //     winner_board, winner_step, lottery_nums[winner_step]
    // );
    // println!("boards: {:?}", boards[winner_board]);
    // println!("boards result: {:?}", boards_result[winner_board]);
    let mut sum = 0;
    let marked_lottery_nums = &lottery_nums[..winner_step + 1];
    for row in &boards[winner_board] {
        for val in row {
            if !marked_lottery_nums.contains(&val) {
                sum += val;
                // print!("{},", val);
            }
        }
    }
    // println!("unmarked sum: {}", sum);
    sum * lottery_nums[winner_step]
}

fn day4p2(lottery_nums: &[i32], boards: Vec<Vec<Vec<i32>>>) -> i32 {
    let board_size = boards.len();

    let mut boards_result = Vec::new();
    let board_result = vec![vec![0; 5], vec![0; 5], vec![0], vec![0]];
    for _ in 0..board_size {
        boards_result.push(board_result.clone());
    }

    let mut last_winner_board = board_size;
    let mut winner_board_list = Vec::new();
    let mut winner_step = lottery_nums.len();

    'CHECKING: for idx in 0..lottery_nums.len() {
        winner_step = idx;
        for (board_idx, board) in boards.iter().enumerate() {
            if winner_board_list.contains(&board_idx) {
                continue;
            }

            for (i, row) in board.iter().enumerate() {
                for (j, &col) in row.iter().enumerate() {
                    if col == lottery_nums[idx] {
                        //row
                        boards_result[board_idx][0][i] += 1;
                        if boards_result[board_idx][0][i] == 5 {
                            winner_board_list.push(board_idx);
                            continue;
                        }
                        //col
                        boards_result[board_idx][1][j] += 1;
                        if boards_result[board_idx][1][j] == 5 {
                            winner_board_list.push(board_idx);
                            continue;
                        }
                    }

                    if winner_board_list.len() == board_size - 1 {
                        last_winner_board = board_idx;
                        break 'CHECKING;
                    }
                }
            }
        }
    }

    // println!(
    //     "FOUND! Winner: {} with steps: {}, final lottory: {}",
    //     last_winner_board, winner_step, lottery_nums[winner_step]
    // );
    // println!("boards: {:?}", boards[last_winner_board]);
    // println!("boards result: {:?}", boards_result[last_winner_board]);

    let mut sum = 0;
    let marked_lottery_nums = &lottery_nums[..winner_step + 1];
    for row in &boards[last_winner_board] {
        for val in row {
            if !marked_lottery_nums.contains(&val) {
                sum += val;
                // print!("{},", val);
            }
        }
    }
    // println!("unmarked sum: {}", sum);
    sum * lottery_nums[winner_step]
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn day4p1v1() {
//         let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
//         let result = day4p1(&input);
//         let expect = 7;
//         assert_eq!(result, expect);
//     }
//     #[test]
//     fn day4p2v1() {
//         let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
//         let result = day4p2(&input);
//         let expect = 5;
//         assert_eq!(result, expect);
//     }
// }
