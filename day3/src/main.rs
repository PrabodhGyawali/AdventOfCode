use std::{fs, iter};
use regex::Regex;

fn main() {
    let mut total = 0;
    /* Use Regex in Rust */
    let contents = fs::read_to_string("input.txt").unwrap();
    let contents: &str = &contents;
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    
    for (_, [num1, num2]) in re.captures_iter(contents).map(|c|c.extract()) {
        let num1: i32 = num1.parse().unwrap();
        let num2: i32 = num2.parse().unwrap();
        total += num1 * num2;
    };
    println!("{}", total);
}
