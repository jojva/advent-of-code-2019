use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut fuel = 0;
    for line in input.lines() {
        let mass: i32 = line.parse()?;
        fuel += fuel_from_mass(mass);
    }
    writeln!(io::stdout(), "{}", fuel)?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut fuel = 0;
    for line in input.lines() {
        let mass: i32 = line.parse()?;
        fuel += fuel_from_mass_adjusted(mass);
    }
    writeln!(io::stdout(), "{}", fuel)?;
    Ok(())
}

fn fuel_from_mass(mass: i32) -> i32 {
    (mass / 3) - 2
}

fn fuel_from_mass_adjusted(mass: i32) -> i32 {
    let fuel = fuel_from_mass(mass);
    if fuel <= 0 {
        0
    } else {
        fuel + fuel_from_mass_adjusted(fuel)
    }
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

    #[test]
    fn test_fuel_adjusted() {
        assert_eq!(fuel_from_mass_adjusted(14), 2);
        assert_eq!(fuel_from_mass_adjusted(1969), 966);
        assert_eq!(fuel_from_mass_adjusted(100756), 50346);
    }
}
