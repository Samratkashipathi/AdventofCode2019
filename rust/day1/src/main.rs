use std::fs;
use std::io::{BufRead, BufReader};

fn main() {
    let file = fs::File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut total_fuel: f64 = 0.0;

    for item in reader.lines() {
        let mass: i64 = item.unwrap().parse().unwrap();
        let fuel = (mass as f64 / 3.0).floor() - 2.0;
        total_fuel = total_fuel + fuel
    }

    println!("{:#?}", total_fuel)
}