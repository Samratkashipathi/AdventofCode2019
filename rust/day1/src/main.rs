use std::any::Any;
use std::fs;
use std::io::{BufRead, BufReader};

fn part1(file_name: &str) -> f64 {
    let file = fs::File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    let mut total_fuel: f64 = reader.lines()
        .map(|x| x.unwrap().parse().unwrap())
        .map(|x: i64| x as f64)
        .map(|x| (x / 3.0).floor())
        .map(|x| x - 2.0).sum();

    total_fuel
}

fn part2(file_name: &str) -> f64 {
    let file = fs::File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    let mut total_fuel: f64 = 0.0;

    let mut fuel: Vec<f64> = reader.lines()
        .map(|x| x.unwrap().parse().unwrap())
        .map(|x: i64| x as f64).collect();

    // TODO: Find better way to solve below code
    while fuel.len() > 0 {
        fuel = fuel.iter()
            .map(|x| (x / 3.0).floor())
            .map(|x| x - 2.0)
            .filter(|x| x > &0.0).collect();

        let temp: f64 = fuel.iter().sum();
        total_fuel += temp;
    }

    total_fuel
}

fn main() {
    let part1_ans = part1("input.txt");
    println!("Part 1 : {:#?}", part1_ans);
    let part2_ans = part2("input.txt");
    println!("Part 2 : {:#?}", part2_ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_cases() {
        assert_eq!(part2("sample_input.txt"), 50346.0);
    }
}