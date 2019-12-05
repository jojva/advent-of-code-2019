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

fn part2(_input: &str) -> Result<()> {
    Ok(())
}

fn is_password(number: u32) -> bool {
    let digits = number.to_string().chars().collect::<Vec<char>>();
    let mut found_pair = false;
    for digit_pair in digits.windows(2) {
        if digit_pair[0] > digit_pair[1] {
            return false;
        }
        if digit_pair[0] == digit_pair[1] {
            found_pair = true;
        }
    }
    found_pair
}

fn is_actually_password(number: u32) -> bool {
    let digits = number.to_string().chars().collect::<Vec<char>>();
    let mut prev_pair = ('0', '0');
    let mut found_pure_pair = false;
    for digit_pair in digits.windows(2) {
        let curr_pair = (digit_pair[0], digit_pair[1]);
        if curr_pair.0 > curr_pair.1 {
            return false;
        }
        if curr_pair.0 == curr_pair.1 && curr_pair != prev_pair {
            found_pure_pair = true;
        }
    }
    found_pure_pair
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
