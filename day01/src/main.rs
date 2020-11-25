use clap::{crate_description, App, Arg};
use day01::{part1, part2, Movement};
use std::fs::read_to_string;
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
    let movements = read_input(args.value_of("INPUT").unwrap());
    println!("Part 1: {}", part1(&movements));
    if let Some(answer) = part2(&movements) {
        println!("Part 2: {}", answer);
    } else {
        println!("Part 2: not found");
    }
}

fn read_input(filename: &str) -> Vec<Movement> {
    let input = read_to_string(filename).unwrap_or_else(|err| {
        println!("Failed to read file '{}': {}", filename, err.to_string());
        exit(2);
    });
    input
        .split(',')
        .map(|s| s.trim().parse())
        .collect::<Result<_, _>>()
        .unwrap_or_else(|err| {
            println!("Failed to parse input: {}", err);
            exit(3);
        })
}
