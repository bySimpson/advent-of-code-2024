use anyhow::Result;
use clap::{arg, Parser};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use rayon::prelude::*;
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

fn is_valid(point: (isize, isize), col_size: isize, row_size: isize) -> bool {
    if point.0 >= 0 && point.0 < col_size && point.1 >= 0 && point.1 < row_size {
        return true;
    }
    false
}

fn calc_part_01(antennas: HashMap<char, Vec<(isize, isize)>>, col_size: isize, row_size: isize) -> i32 {
    let mut out = HashSet::new();
    antennas.iter().for_each(|(_, antenna)| {
       antenna.iter().tuple_combinations::<(_, _)>().for_each(|(l, r)| {
           let r_to_l = ((l.0 - r.0)*2, (l.1 - r.1)*2);
           let l_to_r = ((r.0 - l.0)*2, (r.1 - l.1)*2);
           let point_1 = (l_to_r.0+l.0, l_to_r.1+l.1);
           let point_2 = (r_to_l.0+r.0, r_to_l.1+r.1);
           if is_valid(point_1, col_size, row_size) {
               out.insert(point_1);
           }
           if is_valid(point_2, col_size, row_size) {
               out.insert(point_2);
           }
       })
    });
    out.len() as i32
}

fn main() -> Result<()> {
    let args = Args::parse();
    let file = File::open(args.path)?;
    let reader = BufReader::new(file);
    let mut antennas: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    let mut col_size = 0;
    let mut row_size = 0;
    for (row, line) in reader.lines().map_while(Result::ok).enumerate() {
        col_size = line.chars().count();
        for (col, char) in line.chars().enumerate() {
            match char {
                '.' => (),
                any => {
                    antennas.entry(any).or_insert_with(Vec::new).push((col as isize, row as isize));
                }
            }
        }
        row_size +=1;
    }

    let part_01 = calc_part_01(antennas, col_size as isize, row_size as isize);
    
    println!("Part_01: {}", part_01);
    println!("Part_02: {}", 0);

    Ok(())
}
