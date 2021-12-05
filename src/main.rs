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
        // "1" => match part {
        //     "1" => days::day1::day1_part1(from_file != 0),
        //     "2" => days::day1::day1_part2(),
        //     _ => (),
        // },
        _ => (),
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    // match matches.occurrences_of("debug") {
    //     0 => println!("Debug mode is off"),
    //     1 => println!("Debug mode is kind of on"),
    //     2 => println!("Debug mode is on"),
    //     _ => println!("Don't be crazy"),
    // }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level app
    // if let Some(matches) = matches.subcommand_matches("test") {
    //     // "$ myapp test" was run
    //     if matches.is_present("list") {
    //         // "$ myapp test -l" was run
    //         println!("Printing testing lists...");
    //     } else {
    //         println!("Not printing testing lists...");
    //     }
    // }

    // Continued program logic goes here...
}
