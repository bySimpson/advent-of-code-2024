use anyhow::Result;
use clap::{arg, Parser};
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum Field {
    Empty,
    Walked,
    Obstacle,
    PlacedObstacle,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Direction {
    pub fn get_change(&self) -> (isize, isize) /*(col, row)*/ {
        match &self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }
}

fn get_at_pos(lines: &[Vec<Field>], column: isize, row: isize) -> Option<Field> {
    if lines.get(row as usize).is_some() && lines.get(row as usize).unwrap().get(column as usize).is_some() {
        return Some(lines[row as usize][column as usize]);
    }
    None
}

fn play_part_01(lines: &mut [Vec<Field>], start_column: isize, start_row: isize, start_direction: Direction, max_steps: isize) -> (isize, bool) { // fields, exited after
    let mut playing = true;
    let mut c_dir = start_direction;
    let mut c_pos = (start_column, start_row);
    let mut counter = 0;
    // set first Field as walked!
    lines[c_pos.1 as usize][c_pos.0 as usize] = Field::Walked;
    while playing {
        let c_change = c_dir.get_change();
        match get_at_pos(lines, c_pos.0 + c_change.0, c_pos.1 + c_change.1) {
            Some(Field::Obstacle) | Some(Field::PlacedObstacle) => {
                c_dir = match c_dir {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                }
            },
            None => playing = false,
            Some(_) => {
                c_pos = (c_pos.0 + c_change.0, c_pos.1 + c_change.1);
                lines[c_pos.1 as usize][c_pos.0 as usize] = Field::Walked;
            }
        }
        counter += 1;
        if counter >= max_steps {
            break;
        }
    }
    (lines.iter().map(|c_line| c_line.iter().filter(|i| **i == Field::Walked).count() as isize).sum::<isize>(),
    !playing)
}

fn play_part_02(lines: &Vec<Vec<Field>>, start_column: isize, start_row: isize) -> isize {
    let mut playing = true;
    let mut c_dir = Direction::Up;
    let mut c_pos = (start_column, start_row);
    let mut out = 0;
    let mut placed_obstacles: HashSet<(isize, isize)> = HashSet::new();
    // set first Field as walked!
    while playing {
        let c_change = c_dir.get_change();
        match get_at_pos(lines, c_pos.0 + c_change.0, c_pos.1 + c_change.1) {
            Some(Field::Obstacle) | Some(Field::PlacedObstacle) => {
                c_dir = match c_dir {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                }
            },
            None => playing = false,
            Some(_) => {
                if !placed_obstacles.contains(&(c_pos.0 + c_change.0, c_pos.1 + c_change.1)) {
                    placed_obstacles.insert((c_pos.0 + c_change.0, c_pos.1 + c_change.1));
                    let mut c_lines = lines.clone();
                    c_lines[(c_pos.1 + c_change.1) as usize][(c_pos.0 + c_change.0) as usize] = Field::PlacedObstacle;
                    let (_, found) = play_part_01(&mut c_lines, start_column, start_row, Direction::Up, 60000);
                    out += !found as isize;
                }
                c_pos = (c_pos.0 + c_change.0, c_pos.1 + c_change.1);
            }
        }
    }
    out
}

fn main() -> Result<()> {
    let args = Args::parse();
    let file = File::open(args.path)?;
    let mut lines: Vec<Vec<Field>> = vec![];
    let reader = BufReader::new(file);
    let mut guard_pos = (0usize, 0usize);
    for (row, line) in reader.lines().map_while(Result::ok).enumerate() {
        let mut c_row = vec![];
        for (col, char) in line.chars().enumerate() {
            let field = match char {
                '#' => Field::Obstacle,
                '^' => {
                    guard_pos = (col, row);
                    Field::Empty
                }
                _ => Field::Empty,
            };
            c_row.push(field);
        }
        lines.push(c_row);
    }
    let part_01 = play_part_01(&mut lines, guard_pos.0 as isize, guard_pos.1 as isize, Direction::Up, 2000);
    
    let part_02 = play_part_02(&mut lines.clone(), guard_pos.0 as isize, guard_pos.1 as isize);
    
    println!("Part_01: {}", part_01.0);
    println!("Part_02: {}", part_02);

    Ok(())
}
