use std::cmp;
#[warn(dead_code)]
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Lines, Result};

fn read_line(file_path: &str) -> Result<Lines<BufReader<File>>> {
    let file = File::open(file_path)?;
    Ok(BufReader::new(file).lines())
}

// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green

struct ColorData {
    value: u32,
    min: u32,
}

enum COLORS {
    RED(ColorData),
    BLUE(ColorData),
    GREEN(ColorData),
}

// const RED_CUBES: u32 = 12;
// const BLUE_CUBES: u32 = 14;
// const GREEN_CUBES: u32 = 13;
impl COLORS {
    fn update(&mut self, value: u32) {
        match self {
            COLORS::BLUE(v) => {
                v.min = cmp::max(value, v.min);
                v.value = value;
            }
            COLORS::GREEN(v) => {
                v.min = cmp::max(value, v.min);
                v.value = value;
            }
            COLORS::RED(v) => {
                v.min = cmp::max(value, v.min);
                v.value = value;
            }
        }
    }
    fn min(&self) -> u32 {
        match self {
            COLORS::BLUE(v) => v.min,
            COLORS::GREEN(v) => v.min,
            COLORS::RED(v) => v.min,
        }
    }
    // fn check(&self) -> bool {
    //     match self {
    //         COLORS::BLUE(v) => *v <= BLUE_CUBES,
    //         COLORS::GREEN(v) => *v <= GREEN_CUBES,
    //         COLORS::RED(v) => *v <= RED_CUBES,
    //     }
    // }
}

fn solve(line: String) -> u32 {
    let game_string: Vec<&str> = line.trim().split(":").collect();
    // let game: Vec<&str> = game_string[0].trim().split(" ").collect();
    // let mut game_number = 0;
    // match game[1].trim().parse() {
    //     Ok(n) => game_number = n,
    //     Err(other) => {
    //         println!("Failed to parse game number'{:?}'", other);
    //     }
    // };

    let reveal_string = game_string[1];

    let reveal_array: Vec<&str> = reveal_string.trim().split(";").collect();

    // let init_struct = ColorData { value: 0, min: 0 };
    let mut red_color = COLORS::RED(ColorData { value: 0, min: 0 });
    let mut blue_color = COLORS::BLUE(ColorData { value: 0, min: 0 });
    let mut green_color = COLORS::GREEN(ColorData { value: 0, min: 0 });

    for reveal in reveal_array {
        let in_reveals: Vec<&str> = reveal.trim().split(",").collect();

        for i_reveal in in_reveals {
            let arr: Vec<&str> = i_reveal.trim().split(" ").collect();
            let mut add_num: u32 = 0;

            match arr[0].trim().parse() {
                Ok(n) => add_num = n,
                Err(other) => {
                    println!("Failed to parse number'{:?}'", other);
                }
            };
            match arr[1].trim() {
                "blue" => blue_color.update(add_num),
                "green" => green_color.update(add_num),
                "red" => red_color.update(add_num),
                _ => panic!("Unknown color {}", arr[1].trim()),
            }
            // let color = match arr[1].trim() {
            //     "blue" => COLORS::BLUE(add_num),
            //     "green" => COLORS::GREEN(add_num),
            //     "red" => COLORS::RED(add_num),
            //     _ => panic!("Unknown color {}", arr[1].trim()),
            // };
            // if color.check() == false {
            //     return 0;
            // }
        }
    }
    red_color.min() * green_color.min() * blue_color.min()
}

fn main() {
    match read_line("C://Users/omers/OneDrive/Documents/rust/aoc_2/src/input.txt") {
        Ok(lines) => {
            let mut ans = 0;
            let mut i = 0;
            for line in lines {
                if let Ok(ip) = line {
                    ans += solve(ip.clone());
                    println!("Iteration {}, Line {}, Answer {}", i, ip.clone(), ans);
                }
                i += 1;
            }
        }
        Err(e) => println!("Error reading files: {}", e),
    }
}
