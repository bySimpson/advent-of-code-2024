use anyhow::Result;
use clap::{arg, Parser};
use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::string::String;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,
    #[arg(short, long)]
    debug: bool,
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

    let mut part_01 = 0;
    let mut part_02 = 0;
    for c_page in page_numbers {
        let valid_part_01 = c_page
            .iter()
            .tuple_windows()
            .map(|(a, b)| orderings.contains(&format!("{}|{}", a, b).to_string()))
            .all(|c| c);
        if valid_part_01 {
            part_01 += c_page[c_page.len() / 2];
        } else {
            let mut ordering = true;
            let mut to_sort = c_page.clone();
            while ordering {
                let mut temp_sort = to_sort.clone();
                ordering = false;
                for (a, b) in to_sort.iter().tuple_windows() {
                    if !orderings.contains(&format!("{}|{}", a, b).to_string()) {
                        let pos = temp_sort.iter().position(|item| item == a).unwrap();
                        let removed = temp_sort.remove(pos);
                        temp_sort.insert(pos + 1, removed);
                        ordering = true;
                        break;
                    }
                }
                to_sort = temp_sort;
            }
            part_02 += to_sort[to_sort.len() / 2];
        }
    }
    println!("Part_01: {}", part_01);
    println!("Part_02: {}", part_02);

    Ok(())
}
