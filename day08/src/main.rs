use clap::{crate_description, value_t_or_exit, App, Arg};
use day08::{Instruction, Screen};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

fn main() {
    let args = App::new(crate_description!())
        .arg(
            Arg::with_name("NUM_COLS")
                .help("Number of columns in the screen")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("NUM_ROWS")
                .help("Number of rows in the screen")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(3),
        )
        .get_matches();

    println!(crate_description!());
    let num_cols = value_t_or_exit!(args.value_of("NUM_COLS"), usize);
    let num_rows = value_t_or_exit!(args.value_of("NUM_ROWS"), usize);
    let instructions = read_input(args.value_of("INPUT").unwrap());

    let mut screen = Screen::new(num_cols, num_rows);
    screen.execute(&instructions);
    println!("Part 1: {}", screen.count_lit_pixels());
    println!("Part 2:\n{}", screen);
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
