use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::{arg, Parser};
use anyhow::{Result};
use std::string::String;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,
    #[arg(short, long)]
    debug: bool
}
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Mode {
    GREATER,
    SMALLER
}

fn is_valid(input: Vec<i32>) -> bool {
    let mut last: Option<i32> = None;
    let mut set_mode: Option<Mode> = None;
    let mut valid = true;
    for current in input {
        if let Some(last) = last {
            let diff = current.abs_diff(last);
            if 0 == diff || diff > 3 {
                valid = false;
                break;
            }

            let current_mode = if current > last {
                Mode::GREATER
            } else {
                Mode::SMALLER
            };
            if let Some(set_mode) = set_mode {
                if set_mode != current_mode {
                    valid = false;
                    break;
                }
            } else {
                set_mode = Some(current_mode);
            }
        }
        last = Some(current);
    }
    valid

}

fn main() -> Result<()> {
    let args = Args::parse();
    let file = File::open(args.path)?;
    let reader = BufReader::new(file);

    let mut part_01 = 0;
    let mut part_02 = 0;
    for line in reader.lines().map_while(Result::ok) {
        let numbers = line.split(' ').map(|i| i.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        
        if !is_valid(numbers.clone()) {
            for i in 0..numbers.len() {
                let mut c_numbers = numbers.clone();
                c_numbers.remove(i);
                if is_valid(c_numbers) {
                    part_02 += 1;
                    break;
                }
            }
        } else {
            part_02 += 1;
            part_01 += 1;
        }
    }
    println!("Part1: {}", part_01);
    println!("Part2: {}", part_02);

    Ok(())
}