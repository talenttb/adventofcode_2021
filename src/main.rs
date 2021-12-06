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
    let day = format!("day{}", matches.value_of("day").unwrap());

    match day.as_ref() {
        "day1" => days::day1::solution(part, from_file != 0),
        "day2" => days::day2::solution(part, from_file != 0),
        "day3" => days::day3::solution(part, from_file != 0),
        "day4" => days::day4::solution(part, from_file != 0),
        "day5" => days::day5::solution(part, from_file != 0),
        "day6" => days::day6::solution(part, from_file != 0),
        _ => (),
    }
}
