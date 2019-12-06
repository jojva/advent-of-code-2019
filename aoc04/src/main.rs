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
    let range: Vec<u32> = input
        .trim()
        .split('-')
        .map(|v| v.parse().unwrap())
        .collect();
    let min = range[0];
    let max = range[1];
    let mut passwords = 0;
    for number in min..=max {
        if is_password(number) {
            passwords += 1;
        }
    }
    writeln!(io::stdout(), "{}", passwords)?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let range: Vec<u32> = input
        .trim()
        .split('-')
        .map(|v| v.parse().unwrap())
        .collect();
    let min = range[0];
    let max = range[1];
    let mut passwords = 0;
    for number in min..=max {
        if is_actually_password(number) {
            passwords += 1;
        }
    }
    writeln!(io::stdout(), "{}", passwords)?;
    Ok(())
}

fn is_password(number: u32) -> bool {
    let digits = [
        (number / 100_000) % 10,
        (number / 10_000) % 10,
        (number / 1_000) % 10,
        (number / 100) % 10,
        (number / 10) % 10,
        (number / 1) % 10,
    ];
    let mut occurrences: [u32; 10] = [0; 10];
    let mut last_digit = 0;
    for &current_digit in &digits {
        if last_digit > current_digit {
            return false;
        }
        occurrences[current_digit as usize] += 1;
        last_digit = current_digit;
    }
    occurrences.iter().any(|x| x >= &2)
}

fn is_actually_password(number: u32) -> bool {
    let digits = [
        (number / 100_000) % 10,
        (number / 10_000) % 10,
        (number / 1_000) % 10,
        (number / 100) % 10,
        (number / 10) % 10,
        (number / 1) % 10,
    ];
    let mut occurrences: [u32; 10] = [0; 10];
    let mut last_digit = 0;
    for &current_digit in &digits {
        if last_digit > current_digit {
            return false;
        }
        occurrences[current_digit as usize] += 1;
        last_digit = current_digit;
    }
    occurrences.iter().any(|x| x == &2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_password() {
        assert_eq!(is_password(100000), false);
        assert_eq!(is_password(123456), false);
        assert_eq!(is_password(123345), true);
    }

    #[test]
    fn test_is_actually_password() {
        assert_eq!(is_actually_password(112233), true);
        assert_eq!(is_actually_password(123444), false);
        assert_eq!(is_actually_password(111122), true);
        assert_eq!(is_actually_password(113444), true);
    }
}
