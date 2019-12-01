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
        let mass: u32 = line.parse()?;
        fuel += fuel_from_mass(mass);
    }
    writeln!(io::stdout(), "{}", fuel)?;
    Ok(())
}

fn part2(_input: &str) -> Result<()> {
    // TODO
    Ok(())
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
