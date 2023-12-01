use std::fs;

fn day1() {
    let data = fs::read_to_string("data/day1.in").unwrap();
    let lines =data.lines();
    let mut sum = 0;
    lines.for_each(|line| sum += line.chars().find(|&c| c.is_ascii_digit()).unwrap().to_digit(10).unwrap() * 10 + line.chars().rev().find(|&c| c.is_ascii_digit()).unwrap().to_digit(10).unwrap());
    println!("{}", sum);
}
fn main() {
   day1();
}
