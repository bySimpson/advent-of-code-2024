use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::{arg, Parser};
use anyhow::{Result};
use std::string::String;
use itertools::Itertools;

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
    let mut order_mode = true;
    let mut orderings: HashSet<String> = HashSet::new();
    let mut page_numbers: Vec<Vec<u32>> = vec![];
    for line in reader.lines().map_while(Result::ok) {
        if line == "" {
            order_mode = false;
            continue;
        }
        if order_mode {
            orderings.insert(line);
        } else {
            let mut c_pages = vec![];

            for nmbr in line.split(',') {
                c_pages.push(nmbr.parse::<u32>()?);
            }

            page_numbers.push(c_pages);
        }
    }

    let mut out = 0;
    for c_page in page_numbers {
         let valid = c_page.iter().tuple_windows().map(|(a,b)| {
             orderings.contains(&format!("{}|{}", a, b).to_string())
         }).all(|c| c);
        if valid {
            out += c_page[c_page.len() / 2];
        }
    }
    println!("Part_01: {}", out);
    println!("Part_02: {}", 0);

    Ok(())
}