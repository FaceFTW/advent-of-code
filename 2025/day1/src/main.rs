use std::{
    fs::File,
    io::Read,
    ops::{Add, Sub},
};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct SafeNumber(u16);

impl Add for SafeNumber {
    type Output = SafeNumber;

    fn add(self, rhs: Self) -> Self::Output {
        let acc = self.0;
        let mut rhs_acc = rhs.0;
        while rhs_acc >= 100 {
            rhs_acc = rhs_acc - 100;
        }

        if acc + rhs_acc > 100 {
            SafeNumber(acc - (100 - rhs_acc))
        } else {
            SafeNumber(acc + rhs_acc)
        }
    }
}

impl Sub for SafeNumber {
    type Output = SafeNumber;

    fn sub(self, rhs: Self) -> Self::Output {
        let acc = self.0;
        let mut rhs_acc = rhs.0;
        while rhs_acc >= 100 {
            rhs_acc = rhs_acc - 100;
        }

        if rhs_acc >= acc {
            SafeNumber(acc + 99 - rhs_acc)
        } else {
            SafeNumber(acc - rhs_acc)
        }
    }
}

#[derive(Debug)]
struct Instruction {
    dir: Direction,
    len: SafeNumber,
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
            println!("{raw}");
            let dir = match raw.chars().nth(0) {
                Some('L') => Direction::Left,
                Some('R') => Direction::Right,
                _ => panic!("Malformed instruction-Unexpected Direction"),
            };

            let num_only = raw.split_at(1).1.trim();
            println!("{num_only}");
            let len = SafeNumber(
                u16::from_str_radix(num_only, 10).expect("Malformed Instruction - bad number!"),
            );
            Instruction { dir, len }
        })
        .collect()
}

fn part1(data: &[Instruction]) {
    let mut acc = 0;
    let mut safe_state = SafeNumber(50);

    for instr in data {
        println!("Current State: {}, Instruction: {:?}", safe_state.0, instr);
        match instr.dir {
            Direction::Left => safe_state = safe_state - instr.len,
            Direction::Right => safe_state = safe_state + instr.len,
        }
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
