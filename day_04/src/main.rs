use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::{arg, Parser};
use anyhow::{Result};
use std::string::String;
use regex::Regex;
use crate::Field::OTHER;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,
    #[arg(short, long)]
    debug: bool
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum Field {
    X,
    M,
    A,
    S,
    OTHER(char)
}

fn is_correct_position(field: &Field, pos: isize) -> bool {
    let out = match pos {
        0 => field == &Field::X,
        1 => field == &Field::M,
        2 => field == &Field::A,
        3 => field == &Field::S,
        other => false
    };
    
    //println!("{}", out);
    out
}

fn get_at_pos(lines: &[Vec<Field>], column: isize, row: isize) -> Option<Field> {
    if lines.get(row as usize).is_some() && lines.get(row as usize).unwrap().get(column as usize).is_some() {
        return Some(lines[row as usize][column as usize]);   
    }
    None
}

fn check_change(lines: &[Vec<Field>], column: isize, row: isize, change_col: isize, change_row: isize) -> bool {
    let mut out = true;
    for i in 0..4 {
        if let Some(field) = get_at_pos(lines, column+(change_col*i), row+(change_row*i)) {
            out &= is_correct_position(&field, i);
        } else {
            out = false;
        }
    }
    out
}


fn enumerate(lines: &[Vec<Field>], column: isize, row: isize) -> isize {
    let mut out = check_change(lines, column, row, 1, 0) as isize; // right
    out += check_change(lines, column, row, -1, 0) as isize; // left
    out += check_change(lines, column, row, 0, 1) as isize; // down
    out += check_change(lines, column, row, 0, -1) as isize; // up
    out += check_change(lines, column, row, 1, 1) as isize; // down right
    out += check_change(lines, column, row, 1, -1) as isize; // down left
    out += check_change(lines, column, row, -1, -1) as isize; // up left
    out += check_change(lines, column, row, -1, 1) as isize; // up right
    
    out
}

fn main() -> Result<()> {
    let args = Args::parse();
    let file = File::open(args.path)?;
    let reader = BufReader::new(file);
    let mut lines: Vec<Vec<Field>> = vec![];
    for line in reader.lines().map_while(Result::ok) {
        let mut c_row = vec![];
        for char in line.chars() {
            let field = match char {
                'X' => Field::X,
                'M' => Field::M,
                'A' => Field::A,
                'S' => Field::S,
                c => Field::OTHER(c)
            };
            c_row.push(field);
        }
        lines.push(c_row);
    }
    let mut found_times: isize = 0;
    for (c_row_place, c_row) in &mut lines.iter().enumerate() {
        for (c_col_place, c_col) in c_row.iter().enumerate() {
            found_times += enumerate(&lines, c_row_place as isize, c_col_place as isize);
        }
    }
    println!("Part_01: {}", found_times);

    Ok(())
}