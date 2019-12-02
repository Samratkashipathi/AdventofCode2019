use std::fs;
use std::io::{BufRead, BufReader};

fn main() {
    let file = fs::File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut total_fuel: f64 = reader.lines()
        .map(|x| x.unwrap().parse().unwrap())
        .map(|x: i64| x as f64)
        .map(|x| (x / 3.0).floor())
        .map(|x| x - 2.0).sum();

    println!("{:#?}", total_fuel)
}