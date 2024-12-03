use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::{arg, Parser};
use anyhow::{Result};
use std::string::String;
use regex::Regex;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,
    #[arg(short, long)]
    debug: bool
}

fn main() -> Result<()> {
    let args = Args::parse();
    let file = File::open(args.path)?;
    let reader = BufReader::new(file);
    let input = reader.lines().map_while(Result::ok).collect::<String>();
    let re = Regex::new(r"mul\((?<first>\d+),(?<second>\d+)\)").unwrap();
    
    let part_01 = re.captures_iter(input.trim()).map(|v| { 
        let a = v["first"].parse::<i32>().unwrap();
        let b = v["second"].parse::<i32>().unwrap();
        a * b
    }).sum::<i32>();
    
    
    println!("Part1: {}", part_01);
    println!("Part2: {}", 0);

    Ok(())
}