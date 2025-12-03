use std::{fs::File, io::Read};

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct SafeNumber(u16);

impl SafeNumber {
    fn turn_dial(&mut self, dir: Direction, amt: u16) {
        let new_pos = match dir {
            Direction::Left => self.0 as i16 + amt as i16,
            Direction::Right => self.0 as i16 - amt as i16,
        };
        self.0 = new_pos.rem_euclid(100).abs() as u16;
    }
}

#[derive(Debug)]
struct Instruction {
    dir: Direction,
    amt: u16,
}

fn main() {
    let mut string = String::new();
    match File::open("2025/day1/input.txt") {
        Ok(mut file) => {
            let _ = file.read_to_string(&mut string);
        }
        Err(e) => panic!("{e}"),
    }

    let data = parse_data(string.as_str());

    part1(data.as_slice());
    part2();
}

fn parse_data(string: &str) -> Vec<Instruction> {
    string
        .trim_end()
        .split("\n")
        .map(|raw| {
            // println!("{raw}");
            let dir = match raw.chars().nth(0) {
                Some('L') => Direction::Left,
                Some('R') => Direction::Right,
                _ => panic!("Malformed instruction-Unexpected Direction"),
            };

            let num_only = raw.split_at(1).1.trim();
            // println!("{num_only}");
            let amt =
                u16::from_str_radix(num_only, 10).expect("Malformed Instruction - bad number!");
            Instruction { dir, amt }
        })
        .collect()
}

fn part1(data: &[Instruction]) {
    let mut acc = 0;
    let mut safe_state = SafeNumber(50);

    for instr in data {
        println!("Current State: {}\nInstruction: {:?}", safe_state.0, instr);
        safe_state.turn_dial(instr.dir, instr.amt);
        println!("New State: {}", safe_state.0);

        if safe_state.0 == 0 {
            acc += 1;
        }
    }

    println!("Part 1 result: {acc}");
}

fn part2() {
    // println!("Day  Part 2 result: {final_result}");
}
