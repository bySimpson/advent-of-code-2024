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
    let input = reader.lines().map(|c_item| c_item.unwrap()).collect::<Vec<String>>();

    let mut left_list = Vec::new();
    let mut right_list = Vec::new();
    for row in input.into_iter() {
        let splits: Vec<_> = row.split("   ").collect();
        left_list.push(splits[0].parse::<usize>()?);
        right_list.push(splits[1].parse::<usize>()?);
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