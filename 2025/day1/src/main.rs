use std::{
    fs::File,
    io::Read,
    ops::{Add, Sub},
};

enum Direction {
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct SafeNumber(u8);

impl Add for SafeNumber {
    type Output = SafeNumber;

    fn add(self, rhs: Self) -> Self::Output {
        let acc = self.0;
        let mut rhs_acc = rhs.0;
        while rhs_acc >= 100 {
            rhs_acc = rhs_acc - 100;
        }

        if acc + rhs_acc > 100 {
            SafeNumber(rhs_acc - acc)
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

        if acc + rhs_acc > 100 {
            SafeNumber(rhs_acc - acc)
        } else {
            SafeNumber(acc - rhs_acc)
        }
    }
}

struct Instruction(Direction, u8);

fn main() {
    let mut string = String::new();
    match File::open("day/input.txt") {
        Ok(mut file) => {
            let _ = file.read_to_string(&mut string);
        }
        Err(e) => panic!("{e}"),
    }

    let data = parse_data(string.as_str());

    part1();
    part2();
}

fn parse_data(string: &str) -> Vec {
    string.trim_end().split("\n").map(|raw| {}).collect()
}

fn part1() {
    println!("Day  Part 1 result: {final_result}");
}

fn part2() {
    println!("Day  Part 2 result: {final_result}");
}
