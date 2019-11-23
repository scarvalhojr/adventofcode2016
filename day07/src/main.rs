use clap::{crate_description, App, Arg};
use day07::{part1, part2, IP7};
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
    let ips = read_input(args.value_of("INPUT").unwrap());
    println!("Part 1: {}", part1(&ips));
    println!("Part 2: {}", part2(&ips));
}

fn read_input(filename: &str) -> Vec<IP7> {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => {
            println!("Failed to open file '{}': {}", filename, err.to_string());
            exit(2);
        }
    };

    match BufReader::new(file)
        .lines()
        .map(|line| line.and_then(|value| value.parse()))
        .collect()
    {
        Ok(ips) => ips,
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
