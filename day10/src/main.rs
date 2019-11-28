use clap::{crate_description, value_t_or_exit, App, Arg};
use day10::{execute, Instruction, Value};
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
        .arg(
            Arg::with_name("MARKER1")
                .help("First microchip marker")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("MARKER2")
                .help("Second microchip marker")
                .required(true)
                .index(3),
        )
        .get_matches();

    println!(crate_description!());
    let instructions = read_input(args.value_of("INPUT").unwrap());
    let marker1 = value_t_or_exit!(args.value_of("MARKER1"), Value);
    let marker2 = value_t_or_exit!(args.value_of("MARKER2"), Value);
    if let Some((bot, product)) = execute(&instructions, marker1, marker2) {
        println!("Part 1: {}", bot);
        println!("Part 2: {}", product);
    } else {
        println!("Solution not found");
    }
}

fn read_input(filename: &str) -> Vec<Instruction> {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => {
            println!("Failed to open file '{}': {}", filename, err.to_string());
            exit(2);
        }
    };

    match BufReader::new(file)
        .lines()
        .enumerate()
        .map(|(num, line)| {
            line.map_err(|err| (num, err.to_string()))
                .and_then(|value| value.parse().map_err(|err| (num, err)))
        })
        .collect()
    {
        Ok(ips) => ips,
        Err((num, err)) => {
            println!("Failed to parse input file '{}'", filename);
            println!("Line {}: {}", num + 1, err);
            exit(3);
        }
    }
}
