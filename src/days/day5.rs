use crate::days::libs::read_lines;
use regex::Regex;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn is_v_and_h(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }
    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }
    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }
    // fn is_slash(&self) -> bool {
    //     self.is_backslash() || self.is_forward_slash()
    // }

    fn is_backslash(&self) -> bool {
        let x = self.start.x - self.end.x;
        let y = self.start.y - self.end.y;

        if x == y {
            return true;
        }

        false
    }
    fn is_forward_slash(&self) -> bool {
        if self.start.x + self.start.y == self.end.x + self.end.y {
            return true;
        }

        false
    }
}

pub fn solution(part: &str, ff: bool) {
    let mut input = Vec::new();
    let mut size: i32 = 0;
    if ff {
        if let Ok(lines) = read_lines("./src/days/day5input") {
            let re = Regex::new(r"^(\d+),(\d+)\s->\s(\d+),(\d+)$").unwrap();
            let (mut x, mut y): (i32, i32) = (0, 0);
            for line in lines {
                if let Ok(l) = line {
                    let caps = re.captures(&l).unwrap();

                    let s_x = caps[1].parse::<i32>().unwrap();
                    let s_y = caps[2].parse::<i32>().unwrap();
                    let e_x = caps[3].parse::<i32>().unwrap();
                    let e_y = caps[4].parse::<i32>().unwrap();

                    if s_x > x {
                        x = s_x;
                    }
                    if e_x > x {
                        x = s_x;
                    }
                    if s_y > y {
                        y = s_y;
                    }
                    if e_y > y {
                        y = e_y;
                    }

                    input.push(Line {
                        start: Point { x: s_x, y: s_y },
                        end: Point { x: e_x, y: e_y },
                    });
                }
            }
            assert_eq!(x, y);
            size = x + 1;
        }
    } else {
        size = 10;
        input = vec![
            Line {
                start: Point { x: 0, y: 9 },
                end: Point { x: 5, y: 9 },
            },
            Line {
                start: Point { x: 8, y: 0 },
                end: Point { x: 0, y: 8 },
            },
            Line {
                start: Point { x: 9, y: 4 },
                end: Point { x: 3, y: 4 },
            },
            Line {
                start: Point { x: 2, y: 2 },
                end: Point { x: 2, y: 1 },
            },
            Line {
                start: Point { x: 7, y: 0 },
                end: Point { x: 7, y: 4 },
            },
            Line {
                start: Point { x: 6, y: 4 },
                end: Point { x: 2, y: 0 },
            },
            Line {
                start: Point { x: 0, y: 9 },
                end: Point { x: 2, y: 9 },
            },
            Line {
                start: Point { x: 3, y: 4 },
                end: Point { x: 1, y: 4 },
            },
            Line {
                start: Point { x: 0, y: 0 },
                end: Point { x: 8, y: 8 },
            },
            Line {
                start: Point { x: 5, y: 5 },
                end: Point { x: 8, y: 2 },
            },
        ];
    }

    match part {
        "1" => {
            let result = day5p1(size, &input);
            println!("Result: {}", result);
        }
        "2" => {
            let result = day5p2(size, &input);
            println!("Result: {}", result);
        }
        _ => (),
    }
}

fn day5p1(size: i32, arr: &[Line]) -> i32 {
    let mut diagram = vec![vec![0; size as usize]; size as usize];
    // let (mut from, mut to): (i32, i32) = (0, 0);
    let mut from;
    for line in arr {
        if !line.is_v_and_h() {
            continue;
        }
        // println!("{:?}", line);
        // --
        if line.is_horizontal() {
            // println!("{}", (line.end.x - line.start.x).abs());
            from = line.start.x;
            if line.start.x > line.end.x {
                from = line.end.x;
            }

            for i in from..=from + (line.end.x - line.start.x).abs() {
                diagram[line.start.y as usize][i as usize] += 1;
            }
        }
        // |
        else if line.is_vertical() {
            // println!("{}", (line.end.y - line.start.y).abs());
            from = line.start.y;
            if line.start.y > line.end.y {
                from = line.end.y;
            }
            for i in from..=from + (line.end.y - line.start.y).abs() {
                diagram[i as usize][line.start.x as usize] += 1;
            }
        }
    }

    // println!("{:?}", diagram);

    let mut overlap_count = 0;

    for row in diagram {
        for col in row {
            if col >= 2 {
                overlap_count += 1;
            }
        }
    }

    overlap_count
}

fn day5p2(size: i32, arr: &[Line]) -> i32 {
    let mut diagram = vec![vec![0; size as usize]; size as usize];
    // let (mut from, mut to): (i32, i32) = (0, 0);
    let mut from;
    for line in arr {
        // println!("{:?}", line);
        // --
        if line.is_horizontal() {
            // println!("{}", (line.end.x - line.start.x).abs());
            from = line.start.x;
            if line.start.x > line.end.x {
                from = line.end.x;
            }

            for i in from..=from + (line.end.x - line.start.x).abs() {
                diagram[line.start.y as usize][i as usize] += 1;
            }
        }
        // |
        else if line.is_vertical() {
            // println!("{}", (line.end.y - line.start.y).abs());
            from = line.start.y;
            if line.start.y > line.end.y {
                from = line.end.y;
            }
            for i in from..=from + (line.end.y - line.start.y).abs() {
                diagram[i as usize][line.start.x as usize] += 1;
            }
        }
        // /
        else if line.is_forward_slash() {
            // println!("{}", (line.end.x - line.start.x).abs());
            let (mut init_x, mut init_y) = (line.end.x, line.end.y);
            if line.end.x > line.start.x {
                init_x = line.start.x;
                init_y = line.start.y;
            }
            for i in 0..=(line.end.x - line.start.x).abs() {
                diagram[(init_y - i) as usize][(init_x + i) as usize] += 1;
            }
        }
        // \
        else if line.is_backslash() {
            // println!("{}", (line.end.x - line.start.x).abs());
            let (mut init_x, mut init_y) = (line.start.x, line.start.y);
            if line.start.x > line.end.x {
                init_x = line.end.x;
                init_y = line.end.y;
            }
            for i in 0..=(line.end.x - line.start.x).abs() {
                diagram[(init_y + i) as usize][(init_x + i) as usize] += 1;
            }
        }
    }

    // println!("{:?}", diagram);

    let mut overlap_count = 0;

    for row in diagram {
        for col in row {
            if col >= 2 {
                overlap_count += 1;
            }
        }
    }

    overlap_count
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn day5p1v1() {
//         let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
//         let result = day5p1(&input);
//         let expect = 7;
//         assert_eq!(result, expect);
//     }
//     #[test]
//     fn day5p2v1() {
//         let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
//         let result = day5p2(&input);
//         let expect = 5;
//         assert_eq!(result, expect);
//     }
// }
