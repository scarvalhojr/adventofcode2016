use clap::{crate_description, App, Arg};
use day05::{part1, part2};

fn main() {
    let args = App::new(crate_description!())
        .arg(
            Arg::with_name("DOOR_ID")
                .help("The problem input")
                .required(true)
                .index(1),
        )
        .get_matches();

    println!(crate_description!());
    let door_id = args.value_of("DOOR_ID").unwrap();
    println!("Part 1: {}", part1(&door_id));
    println!("Part 2: {}", part2(&door_id));
}
