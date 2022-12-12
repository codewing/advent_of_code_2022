use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use clap::Parser;
use regex::{Regex};
use crate::HandShape::{Paper, Rock, Scissors};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file_path: String,
}

#[derive(Debug)]
enum HandShape {
    Rock,
    Paper,
    Scissors,
}

impl HandShape {
    fn from(text: &str) -> HandShape {
        match text {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
            "X" => Rock,
            "Y" => Paper,
            "Z" => Scissors,
            _ => panic!("Unknown handshape {}", text)
        }
    }

    fn selection_score(&self) -> i32 {
        match *self {
            Rock => 1,
            Paper => 2,
            Scissors => 3
        }
    }

    fn outcome_score(&self, other: &HandShape) -> i32 {
        let outcome = self.selection_score() - other.selection_score();
        let category = (outcome + 3) % 3;
        match category {
            0 => 3,
            1 => 6,
            2 => 0,
            _ => panic!("Missing case {}", category)
        }
    }

    fn find_beating_hand(&self, result: &str) -> HandShape {
        let outcome = self.selection_score() + match result {
            "X" => -1,
            "Y" => 0,
            "Z" => 1,
            _ => panic!("Unknown match result {}", result)
        };
        HandShape::shape_from_value(outcome)
    }

    fn shape_from_value(value: i32) -> HandShape {
        match (value + 3) % 3 {
            0 => Scissors,
            1 => Rock,
            2 => Paper,
            _ => panic!("This shouldn't happen... ({})", value)
        }
    }
}

fn main() {
    let args = Args::parse();

    let mut file = File::open(args.file_path).expect("Couldn't open passed file.");

    assignment_1_calculate_score(&file);
    file.seek(SeekFrom::Start(0)).unwrap();
    assignment_2_calculate_score(&file);
}

fn assignment_1_calculate_score(file: &File) {
    let buffer = BufReader::new(file);
    let re = Regex::new(r"(?P<opponent>[ABC]) (?P<response>[XYZ])").unwrap();

    let mut score = 0;
    for line in buffer.lines().map(|l| l.unwrap()) {
        let captures = re.captures(line.as_str()).unwrap();
        let opponent = HandShape::from(&captures["opponent"]);
        let response = HandShape::from(&captures["response"]);

        score += response.selection_score() + response.outcome_score(&opponent);
    }

    println!("Assignment 1 - Score: {}", score);
}

fn assignment_2_calculate_score(file: &File) {
    let buffer = BufReader::new(file);
    let re = Regex::new(r"(?P<opponent>[ABC]) (?P<result>[XYZ])").unwrap();

    let mut score = 0;
    for line in buffer.lines().map(|l| l.unwrap()) {
        let captures = re.captures(line.as_str()).unwrap();
        let opponent = HandShape::from(&captures["opponent"]);
        let response = opponent.find_beating_hand(&captures["result"]);

        score += response.selection_score() + response.outcome_score(&opponent);
    }

    println!("Assignment 2 - Score: {}", score);
}
