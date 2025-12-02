use std::{fs::File, io::Read};

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Bad value received for direction char"),
        }
    }
}

#[derive(Clone, Debug)]
enum Space {
    Empty,
    Wall,
    Box,
}

struct Data {
    start_pos: (isize, isize),
    map: Vec<Vec<Space>>,
    instructions: Vec<Direction>,
}

fn main() {
    let mut string = String::new();
    match File::open("day15/input.txt") {
        Ok(mut file) => {
            let _ = file.read_to_string(&mut string);
        }
        Err(e) => panic!("{e}"),
    }

    let data = parse_data(string.as_str());

    part1();
    part2();
}

fn parse_data(string: &str) -> Data {
    let (map_str, instr_str) = string.trim_end().split_once("\n\n").unwrap();

    let mut start_pos = (0isize, 0isize); //temp
    let map = map_str
        .split("\n")
        .enumerate()
        .map(|(row_idx, row)| {
            row.chars()
                .enumerate()
                .map(|(col_idx, char)| match char {
                    '#' => Space::Wall,
                    'O' => Space::Box,
                    '@' => {
                        start_pos = (row_idx as isize, col_idx as isize);
                        Space::Empty
                    }
                    _ => Space::Empty, //good fallback
                })
                .collect()
        })
        .collect();

    let instructions = instr_str
        .split("\n")
        .flat_map(|row| {
            row.chars()
                .map(|char| Direction::from(char))
                .collect::<Vec<_>>()
        })
        .collect();

    Data {
        start_pos,
        map,
        instructions,
    }
}

fn part1() {
    println!("Day 15 Part 1 result: {final_result}");
}

fn move_box(pos: (isize, isize), direction: Direction, map: &mut Vec<Vec<Space>>) {
    let map_h = map.len();
    let map_w = map[0].len();
    let (row, col) = pos;
    match direction {
        Direction::Up => {
            if row - 1 >= 0 {
                let new_row = (row - 1) as usize;
                match map[new_row][col] {

                }
            }
        }
        Direction::Down => todo!(),
        Direction::Left => todo!(),
        Direction::Right => todo!(),
    }
}

fn part2() {
    // println!("Day  Part 2 result: {final_result}");
}
