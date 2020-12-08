use clap::{App, Arg};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    let matches = App::new("AOC")
        .version("0.1")
        .author("mari <mari@hakke.ro>")
        .about("AOC 2020 solutions")
        .arg(Arg::new("day")
            .short('d')
            .long("day")
            .required(true)
            .takes_value(true)
            .about("Which day to run"))
        .get_matches();

    let day = matches.value_of("day").unwrap_or("0");

    run(day);
}

fn run(day: &str) {
    match day {
        "1" => day1::run(),
        "2" => day2::run(),
        "3" => day3::run(),
        "4" => day4::run(),
        "5" => day5::run(),
        "6" => day6::run(),
        "7" => day7::run(),
        _   => println!("No such day! :O")
    }
}