use std::fs;
use regex::Regex;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(up|down|forward) (\d+)$").unwrap();
}

fn forward(horizontal: &mut i32, vertical: &mut i32, aim: i32, amount: i32) {
    *horizontal += amount;
    *vertical += aim * amount;
}

fn drive(horizontal: &mut i32, vertical: &mut i32, aim: &mut i32, input: &str) {
    let captures = RE.captures(input).unwrap();

    let digits = captures.get(2).map_or(0, |m| m.as_str().parse::<i32>().unwrap());
    match captures.get(1).map_or("", |m| m.as_str()) {
        "up" => *aim -= digits,
        "down" => *aim += digits,
        "forward" => forward(horizontal, vertical, *aim, digits),
        &_ => panic!("Failed to match")
    };
}


fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut horizontal: i32 = 0;
    let mut vertical: i32 = 0;
    let mut aim: i32 = 0;
    for line in contents.lines() {
        drive(&mut horizontal, &mut vertical, &mut aim, line)
    }

    let result = horizontal * vertical;
    println!("{result}")
}
