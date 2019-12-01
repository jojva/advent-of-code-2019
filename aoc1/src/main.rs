use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("data/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut total_fuel = 0;
    for line in reader.lines() {
        total_fuel += fuel_from_mass(line.unwrap().parse::<u32>().unwrap());
    }
    println!("{}", total_fuel);
}

fn fuel_from_mass(mass: u32) -> u32 {
    (mass / 3) - 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuel() {
        assert_eq!(fuel_from_mass(12), 2);
        assert_eq!(fuel_from_mass(14), 2);
        assert_eq!(fuel_from_mass(1969), 654);
        assert_eq!(fuel_from_mass(100756), 33583);
    }
}
