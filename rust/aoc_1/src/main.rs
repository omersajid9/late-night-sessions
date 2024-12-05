#[warn(dead_code)]
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Lines, Result};

const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn read_lines(file_path: &str) -> Result<Lines<BufReader<File>>> {
    let file = File::open(file_path)?;
    Ok(BufReader::new(file).lines())
}

fn solve(line: String) -> u32 {
    let char_vec: Vec<char> = line.chars().collect();
    let mut v: Vec<u32> = Vec::new();

    for (i, c) in char_vec.iter().enumerate() {
        for (_i, _num) in NUMBERS.iter().enumerate() {
            if _num.len() + i <= line.len() {
                if &line[i.._num.len() + i] == *_num {
                    println!("A: {}, B: {}", &line[i.._num.len() + i], *_num);
                    v.push((_i as u32) + 1);
                }
            }
        }
        if c.is_numeric() {
            v.push(*c as u32 - 0x30);
        }
    }

    let ans = if v.len() == 1 {
        (v[0] * 10) + v[0]
    } else {
        (v[0] * 10) + v[v.len() - 1]
    };

    println!("v: {:?}, answer, {}", v, ans);
    ans
}

fn main() {
    match read_lines("C://Users/omers/OneDrive/Documents/rust/aoc_1/src/input.txt") {
        Ok(lines) => {
            let mut i: u32 = 0;
            let mut z = 0;
            for line in lines {
                if let Ok(ip) = line {
                    i += solve(ip.clone());
                    println!("Iteration {z}, Value {i}, Text {}", ip);
                    z += 1;
                }
            }
            // println!("File contents: \n{}", contents)
        }
        Err(e) => println!("Error reading file: {}", e),
    }
}
