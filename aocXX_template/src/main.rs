use std::{fs::File, io::{BufReader, BufRead, Seek, SeekFrom}};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file_path: String,
}

fn main() {
    let args = Args::parse();
    
    let mut file = File::open(args.file_path).expect("Couldn't open passed file.");

    assignment_1(&file);
    file.seek(SeekFrom::Start(0)).unwrap();
    assignment_2(&file)
}

fn assignment_1(file: &File) {
    let buffer = BufReader::new(file);
    for line in buffer.lines().map(|l| l.unwrap()) {
    
    }
}

fn assignment_2(file: &File) {
    let buffer = BufReader::new(file);
    for line in buffer.lines().map(|l| l.unwrap()) {
    
    }
}
