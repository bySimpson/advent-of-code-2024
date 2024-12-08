use anyhow::Result;
use clap::{arg, Parser};
use itertools::Itertools;
use std::collections::HashSet;
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::string::String;
use crate::Operator::{Multiply, Plus, Combiner};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,
    #[arg(short, long)]
    debug: bool,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Operator {
    Plus = 0,
    Multiply = 1,
    Combiner = 2,
}

impl Operator {
    fn get_num(self) -> usize {
        match self {
            Operator::Plus => 0,
            Operator::Multiply => 1,
            Operator::Combiner => 2,
        }
    }
    
    fn from_num(num: usize, base: usize) -> Self {
        match num % base {
            0 => {
                Plus
            }
            1 => {
                Multiply
            }
            2 => {
                Combiner
            }
            _ => panic!()
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Equation {
    test_value: isize,
    numbers: Vec<isize>
}

impl Equation {
    fn from_line(line: &str) -> Self {
        let mut input_split = line.split(": ");
        let test_value = input_split.next().unwrap().parse::<isize>().unwrap();
        let numbers = input_split.next().unwrap().split(" ").map(|num| num.parse::<isize>().unwrap()).collect();
        Self { test_value, numbers }
    }
    
    fn get_all_opertions(&self, amount_operations: usize) -> Vec<Vec<Operator>> {
        let amount_numbers = self.numbers.len();
        let amount_combinations = amount_operations.pow(amount_numbers as u32) as usize;
        let mut out: Vec<Vec<Operator>> = vec![];
        for _ in 0..amount_combinations {
            out.push(vec![])
        }
        
        for column in 0..amount_numbers {
            for row in 0..amount_combinations {
                out[row].push(Operator::from_num(row / amount_operations.pow(column as u32) % amount_operations, amount_operations));
            }
        }
        
        out
    }
    
    fn calculate_part(&self, base: usize) -> isize {
        let combinations = self.get_all_opertions(base);
        let found = combinations.iter().map( |combination| {
            combination.iter().zip(self.numbers.iter()).fold(0, |acc, (oper, num)| {
                match oper {
                    Plus => {
                        acc + num 
                    }
                    Multiply => {
                        acc * num
                    }
                    Combiner => {
                        format!("{}{}", acc, num).parse::<isize>().unwrap()
                    }
                }
            })
        }).any(|num| num == self.test_value);
        if found {
            self.test_value
        } else {
            0
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let file = File::open(args.path)?;
    let reader = BufReader::new(file);
    let mut tests = Vec::new();
    for line in reader.lines().map_while(Result::ok) {
        tests.push(Equation::from_line(&line));
    }
    let part_01 = tests.clone().par_iter_mut().map(|t| {
        t.calculate_part(2)
    }).sum::<isize>();

    let part_02 = tests.par_iter_mut().map(|t| {
        t.calculate_part(3)
    }).sum::<isize>();

    println!("Part_01: {}", part_01);
    println!("Part_02: {}", part_02);

    Ok(())
}
