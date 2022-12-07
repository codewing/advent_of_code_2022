use std::io::{BufRead, BufReader};
use std::fs::File;
use clap_v3::{App, Arg};

#[derive(Clone, Copy)]
struct Elf {
    index: i32,
    calories: i32
}

fn main() {
    let matches = App::new("myapp")
        .version("1.0")
        .author("Frieder Baumann. <codewing@web.de>")
        .about("Advent of Code - Day 1")
            .arg(Arg::with_name("file").short('f').long("file").value_name("FILE").required(true).takes_value(true).help("Path to the input file"))
        .get_matches();

    let file_path = matches.value_of("file").unwrap();
    let file = File::open(file_path).expect("Couldn't open passed file.");

    let mut current_elf = Elf { index: 0, calories: 0};
    let mut best_elf = Elf { index: 0, calories: 0};
    let buffer = BufReader::new(file);
    for line in buffer.lines().map(|l| l.unwrap()) {
        if line.is_empty() {
            if current_elf.calories > best_elf.calories {
                best_elf = current_elf.clone();
            }
            current_elf.index += 1;
            current_elf.calories = 0;
        } else {
            current_elf.calories += line.parse::<i32>().unwrap();
        }
    }

    println!("Best Elf #{} with {} calories.", best_elf.index, best_elf.calories);
}
