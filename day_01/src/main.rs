use std::collections::HashMap;
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
        left_list.push(splits[0].parse::<isize>()?);
        right_list.push(splits[1].parse::<isize>()?);
    }
    left_list.sort();
    right_list.sort();
    let out = left_list.into_iter().zip(right_list.into_iter()).map(|(l, r)| {
        l.abs_diff(r)
    }).sum::<usize>();
    println!("{}", out);
    Ok(())
}