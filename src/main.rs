use clap::{App, Arg};
// use days::Day1Part1;

mod days;

fn main() {
    let matches = App::new("AOC")
        .arg(Arg::new("fromFile").long("ff"))
        .arg(Arg::new("day").short('d').takes_value(true))
        .arg(Arg::new("part").short('p').takes_value(true))
        .get_matches();

    let from_file = matches.occurrences_of("fromFile");
    let part = matches.value_of("part").unwrap_or_else(|| "1");
    // println!("part: {}", part);
    let day = matches.value_of("day").unwrap();

    match day {
        "1" => days::day1::solution(part, from_file != 0),
        "2" => days::day2::solution(part, from_file != 0),
        "3" => days::day3::solution(part, from_file != 0),
        "4" => days::day4::solution(part, from_file != 0),
        _ => (),
    }
}
