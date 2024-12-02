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

fn main() -> Result<()> {
    let args = Args::parse();
    let file = File::open(args.path)?;
    let reader = BufReader::new(file);
    
    let mut part_01 = 0;
    for line in reader.lines().map_while(Result::ok) {
        let mut last: Option<u32> = None;
        let mut set_mode: Option<Mode> = None;
        let mut valid = true;
        for c_split in line.split(" ") {
            let current = c_split.parse::<u32>()?;
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
                        break
                    }
                } else {
                    set_mode = Some(current_mode);
                }
            }
            last = Some(current);
        }
        part_01 += valid as i32;
    }
    println!("Part1: {}", part_01);

    Ok(())
}