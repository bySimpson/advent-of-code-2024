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


fn main() -> Result<()> {
    let args = Args::parse();
    let file = File::open(args.path)?;
    let reader = BufReader::new(file);

    let mut left_list = Vec::new();
    let mut right_list = Vec::new();
    for line in reader.lines().map_while(Result::ok) {
        let mut splits = line.split("   ");
        left_list.push(splits.next().unwrap().parse::<usize>()?);
        right_list.push(splits.next().unwrap().parse::<usize>()?);
    }
    left_list.sort();
    right_list.sort();
    let day1 = left_list.iter().zip(right_list.iter()).map(|(l, r)| {
        l.abs_diff(*r)
    }).sum::<usize>();
    println!("Day 1: {}", day1);

    let day2 = left_list.iter().map(|l| {
        *l * right_list.iter().filter(|r| **r == *l).count()
    }).sum::<usize>();

    println!("Day 2: {}", day2);
    Ok(())
}