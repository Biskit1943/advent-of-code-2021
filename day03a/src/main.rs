extern crate core;

use std::fs;

#[derive(Clone)]
struct Count {
    zero: i32,
    one: i32,
}

impl Count {
    fn count(&mut self, char: char) {
        let c = char.to_digit(10).unwrap();
        if c == 1 {
            self.one += 1;
        } else {
            self.zero += 1;
        }
    }
}

struct BinaryDiagnostic {
    counts: Vec<Count>,
}

impl BinaryDiagnostic {
    fn count_all(&mut self, input: &str) {
        for (i, c) in input.chars().enumerate() {
            self.counts[i].count(c);
        }
    }

    fn gamma_epsilon_rate(&self) -> (i32, i32) {
        let mut gamma = "".to_owned();
        let mut epsilon = "".to_owned();
        for count in &self.counts {
            if count.zero > count.one {
                gamma.push_str("0");
                epsilon.push_str("1");
            } else if count.one > count.zero {
                gamma.push_str("1");
                epsilon.push_str("0");
            } else {
                panic!("Counts are equal!");
            }
        }
        return (
            i32::from_str_radix(&gamma, 2).expect("Not a binary number!"),
            i32::from_str_radix(&epsilon, 2).expect("Not a binary number!")
        );
    }
}


fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let binary_str_length = contents.lines().nth(0).expect("No lines found!").len();
    let mut binary_diagnostics = BinaryDiagnostic {
        counts: vec![Count { zero: 0, one: 0 }; binary_str_length]
    };

    for line in contents.lines() {
        binary_diagnostics.count_all(line);
    }

    let (gamma_rate, epsilon_rate) = binary_diagnostics.gamma_epsilon_rate();
    let power_consumption = gamma_rate * epsilon_rate;

    println!("{:?}", power_consumption)
}
