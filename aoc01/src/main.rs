use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::fs::File;
use clap::{Parser};

#[derive(Clone, Copy)]
struct Elf {
    index: i32,
    calories: i32
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file_path: String,
}

fn main() {
    let args = Args::parse();

    let mut file = File::open(args.file_path).expect("Couldn't open passed file.");

    assignment_1_calculate_best_elf(&file);

    file.seek(SeekFrom::Start(0)).unwrap();
    assignment_2_calculate_best_elves(&file);
}

fn assignment_1_calculate_best_elf (file: &File) {
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

fn assignment_2_calculate_best_elves (file: &File) {
    let buffer = BufReader::new(file);
    let lines = buffer.lines().map(|l| l.unwrap());
    let mut elves = lines.fold(vec![Elf{index: 0, calories: 0}], |mut elements, current| {
        if current.is_empty() {
            elements.push(Elf {index: elements.len() as i32, calories: 0})
        } else {
            if let Some(last) = elements.last_mut() {
                last.calories += current.parse::<i32>().unwrap();
            }
        }
        elements
    });
    elves.sort_by(|elf1, elf2| { elf2.calories.cmp(&elf1.calories)});

    let calories_total = elves.iter().take(3).fold(0, |acc, elf| acc + elf.calories);

    println!("Top 3 Elves carry {} calories.", calories_total);
}
