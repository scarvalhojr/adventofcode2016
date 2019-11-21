use clap::{crate_description, App, Arg};
use day06::{count_chars, part1, part2};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

fn main() {
    let args = App::new(crate_description!())
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    println!(crate_description!());
    let messages = read_input(args.value_of("INPUT").unwrap());
    let counters = count_chars(&messages);
    println!("Part 1: {}", part1(&counters));
    println!("Part 2: {}", part2(&counters));
}

fn read_input(filename: &str) -> Vec<String> {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => {
            println!("Failed to open file '{}': {}", filename, err.to_string());
            exit(2);
        }
    };

    match BufReader::new(file).lines().collect::<Result<Vec<_>, _>>() {
        Ok(messages) => messages,
        Err(err) => {
            println!(
                "Failed to parse input file '{}': {}",
                filename,
                err.to_string()
            );
            exit(3);
        }
    }
}
