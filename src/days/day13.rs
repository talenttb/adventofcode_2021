use crate::days::libs::read_lines;
use regex::Regex;

pub fn solution(part: &str, ff: bool) {
    let mut input = Vec::new();
    if ff {
        if let Ok(lines) = read_lines("./src/days/day13input") {
            for line in lines {
                if let Ok(l) = line {
                    // println!("{}", l);
                    input.push(l);
                    // input.push(l.parse::<i32>().unwrap());
                    // for v in l.split(",") {
                    //     input.push(v.parse::<i32>().unwrap());
                    // }
                    // let _tmp = input
                    //     .iter()
                    //     .map(|item| {
                    //         item.split("")
                    //             .filter_map(|x| x.parse::<i32>().ok())
                    //             .collect::<Vec<i32>>()
                    //     })
                    //     .collect::<Vec<Vec<i32>>>();
                }
            }
        }
    } else {
        input = vec![
            "6,10".to_string(),
            "0,14".to_string(),
            "9,10".to_string(),
            "0,3".to_string(),
            "10,4".to_string(),
            "4,11".to_string(),
            "6,0".to_string(),
            "6,12".to_string(),
            "4,1".to_string(),
            "0,13".to_string(),
            "10,12".to_string(),
            "3,4".to_string(),
            "3,0".to_string(),
            "8,4".to_string(),
            "1,10".to_string(),
            "2,14".to_string(),
            "8,10".to_string(),
            "9,0".to_string(),
            "".to_string(),
            "fold along y=7".to_string(),
            "fold along x=5".to_string(),
        ];
    }

    match part {
        "1" => {
            let result = day13p1(&input);
            println!("Result: {}", result);
        }
        "2" => {
            let result = day13p2(&input);
            println!("Result: {}", result);
        }
        _ => (),
    }
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

fn convert_input(input: &[String]) -> (usize, usize, Vec<Point>, Vec<Point>) {
    let (papers_str, rest): (Vec<_>, Vec<_>) = input.into_iter().partition(|&x| x.contains(","));
    let (folds_str, _): (Vec<_>, Vec<_>) = rest.into_iter().partition(|&x| x.contains("="));
    let (mut x_size, mut y_size): (usize, usize) = (0, 0);
    let mut points: Vec<Point> = vec![];
    for paper_point in papers_str.iter() {
        let item = paper_point.split(",").collect::<Vec<&str>>();
        let x = item[0].parse::<usize>().unwrap();
        let y = item[1].parse::<usize>().unwrap();
        if x > x_size {
            x_size = x;
        }
        if y > y_size {
            y_size = y;
        }
        points.push(Point { x, y });
    }
    let mut folds: Vec<Point> = vec![];
    let re = Regex::new(r"^fold along (?P<name>[xy])=(\d+)$").unwrap();
    for fold_str in folds_str.iter() {
        let caps = re.captures(fold_str).unwrap();
        let cap1 = caps[1].to_string();
        let cap2 = caps[2].to_string();
        let value = cap2.parse::<usize>().unwrap();
        let fold;
        if cap1 == "x" {
            fold = Point { x: value, y: 0 };
        } else {
            fold = Point { x: 0, y: value };
        }
        folds.push(fold);
    }

    (x_size + 1, y_size + 1, points, folds)
}

fn day13p1(input: &[String]) -> i32 {
    // println!("{:?}", input);
    let (x_size, y_size, points, folds) = convert_input(input);
    // x if for right
    // y if for down
    let mut board = vec![vec!['.'; x_size]; y_size];
    // println!("{:?}", board);
    for point in points {
        board[point.y][point.x] = '#';
    }
    match folds[0] {
        Point { x: 0, y } => {
            println!("match y, y={:?}", y);
            for axie_x in 0..x_size {
                for axie_y in 0..y {
                    // println!(
                    //     "move ({:?},{:?}) to merge ({},{})",
                    //     y_size - axie_y - 1,
                    //     axie_x,
                    //     axie_y,
                    //     axie_x
                    // );
                    board[axie_y][axie_x] = if board[axie_y][axie_x] == '#'
                        || board[y_size - axie_y - 1][axie_x] == '#'
                    {
                        '#'
                    } else {
                        '.'
                    };
                }
            }
            board = board[..y].to_vec();
        }
        Point { x, y: 0 } => {
            println!("match x, x={:?}", x);
            for axie_y in 0..y_size {
                for axie_x in 0..x {
                    // println!(
                    //     "merge ({:?},{:?}) and ({},{})",
                    //     axie_y,
                    //     x_size - axie_x - 1,
                    //     axie_y,
                    //     axie_x,
                    // );
                    board[axie_y][axie_x] = if board[axie_y][axie_x] == '#'
                        || board[axie_y][x_size - axie_x - 1] == '#'
                    {
                        '#'
                    } else {
                        '.'
                    };
                }
            }
            // board = board[..y].to_vec();
            let mut tmp_board: Vec<Vec<char>> = vec![];
            for row in 0..y_size {
                tmp_board.push(board[row][..x].to_vec());
            }
            board = tmp_board;
        }
        _ => (),
    }
    // for fold in folds {
    //     match fold {
    //         Point { x: 0, y } => {
    //             println!("match y, y={:?}", y);
    //             for axie_x in 0..x_size {
    //                 for axie_y in 0..y {
    //                     println!(
    //                         "move ({:?},{:?}) to merge ({},{})",
    //                         y_size - axie_y - 1,
    //                         axie_x,
    //                         axie_y,
    //                         axie_x
    //                     );
    //                     board[axie_y][axie_x] = if board[axie_y][axie_x] == '#'
    //                         || board[y_size - axie_y - 1][axie_x] == '#'
    //                     {
    //                         '#'
    //                     } else {
    //                         '.'
    //                     };
    //                 }
    //             }
    //             board = board[..y].to_vec();
    //         }
    //         Point { x, y: 0 } => {
    //             println!("match x, x={:?}", x);
    //         }
    //         _ => (),
    //     }
    // }

    // println!("{:?}", board);

    let visible_count: usize = board
        .iter()
        .map(|item| item.iter().filter(|&&x| x == '#').collect::<Vec<_>>().len())
        .collect::<Vec<usize>>()
        .iter()
        .sum();
    // println!("{:?}", visible_count);
    visible_count as i32
}

fn day13p2(input: &[String]) -> i32 {
    let (mut x_size, mut y_size, points, folds) = convert_input(input);
    // x if for right
    // y if for down
    let mut board = vec![vec!['.'; x_size]; y_size];
    // println!("{:?}", board);
    for point in points {
        board[point.y][point.x] = '#';
    }
    for fold in folds {
        x_size = board[0].len();
        y_size = board.len();
        match fold {
            Point { x: 0, y } => {
                // println!("match y, y={:?}", y);
                for axie_x in 0..x_size {
                    for axie_y in 0..y {
                        // println!(
                        //     "move ({:?},{:?}) to merge ({},{})",
                        //     y_size - axie_y - 1,
                        //     axie_x,
                        //     axie_y,
                        //     axie_x
                        // );
                        board[axie_y][axie_x] = if board[axie_y][axie_x] == '#'
                            || board[y_size - axie_y - 1][axie_x] == '#'
                        {
                            '#'
                        } else {
                            '.'
                        };
                    }
                }
                board = board[..y].to_vec();
            }
            Point { x, y: 0 } => {
                // println!("match x, x={:?}", x);
                for axie_y in 0..y_size {
                    for axie_x in 0..x {
                        // println!(
                        //     "merge ({:?},{:?}) and ({},{})",
                        //     axie_y,
                        //     x_size - axie_x - 1,
                        //     axie_y,
                        //     axie_x,
                        // );
                        board[axie_y][axie_x] = if board[axie_y][axie_x] == '#'
                            || board[axie_y][x_size - axie_x - 1] == '#'
                        {
                            '#'
                        } else {
                            '.'
                        };
                    }
                }
                // board = board[..y].to_vec();
                let mut tmp_board: Vec<Vec<char>> = vec![];
                for row in 0..y_size {
                    tmp_board.push(board[row][..x].to_vec());
                }
                board = tmp_board;
            }
            _ => (),
        }
    }

    println!("{:?}", board);
    // '.''.''#''#''.''#''#''#''#''.''.''#''#''.''.''#''.''.''#''.''.''#''#''.''.''#''#''#''.''.''#''#''#''.''.''#''#''#''.''.'
    // '.''.''.''#''.''.''.''.''#''.''#''.''.''#''.''#''.''.''#''.''#''.''.''#''.''#''.''.''#''.''#''.''.''#''.''#''.''.''#''.'
    // '.''.''.''#''.''.''.''#''.''.''#''.''.''.''.''#''.''.''#''.''#''.''.''#''.''#''.''.''#''.''#''.''.''#''.''#''#''#''.''.'
    // '.''.''.''#''.''.''#''.''.''.''#''.''#''#''.''#''.''.''#''.''#''#''#''#''.''#''#''#''.''.''#''#''#''.''.''#''.''.''#''.'
    // '#''.''.''#''.''#''.''.''.''.''#''.''.''#''.''#''.''.''#''.''#''.''.''#''.''#''.''.''.''.''#''.''#''.''.''#''.''.''#''.'
    // '.''#''#''.''.''#''#''#''#''.''.''#''#''#''.''.''#''#''.''.''#''.''.''#''.''#''.''.''.''.''#''.''.''#''.''#''#''#''.''.'
    0
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn day13p1v1() {
//         let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
//         let result = day13p1(&input);
//         let expect = 7;
//         assert_eq!(result, expect);
//     }
//     #[test]
//     fn day13p2v1() {
//         let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
//         let result = day13p2(&input);
//         let expect = 5;
//         assert_eq!(result, expect);
//     }
// }
