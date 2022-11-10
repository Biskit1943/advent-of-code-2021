use std::fs;
use std::ops::Index;

fn get_life_support_rating(lines: Vec<&str>, index: usize, common: bool) -> Vec<&str> {
    if lines.len() == 1 || index == lines.index(0).len() {
        return lines;
    }

    let mut zero: Vec<&str> = Vec::new();
    let mut one: Vec<&str> = Vec::new();

    for line in lines {
        match line.chars().nth(index).unwrap() {
            '0' => zero.push(line),
            '1' => one.push(line),
            _ => panic!("Failed to match character")
        }
    }

    if common && zero.len() > one.len() {
        get_life_support_rating(zero, index + 1, common)
    } else if !common && zero.len() <= one.len() {
        get_life_support_rating(zero, index + 1, common)
    } else {
        get_life_support_rating(one, index + 1, common)
    }
}


fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let lines = contents.lines().collect::<Vec<&str>>();
    let oxygen_generator_rating = i32::from_str_radix(
        get_life_support_rating(lines, 0, true).index(0),
        2).unwrap();
    let lines = contents.lines().collect::<Vec<&str>>();
    let co2_scrubber_rating = i32::from_str_radix(
        get_life_support_rating(lines, 0, false).index(0),
        2).unwrap();

    let life_support_rating = oxygen_generator_rating * co2_scrubber_rating;
    println!("{}", life_support_rating);
}
