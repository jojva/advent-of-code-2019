use std::io::{self, Read, Write};
use std::str::FromStr;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let moves = parse_input(input);
    writeln!(io::stdout(), "{:?}", moves)?;
    Ok(())
}

fn parse_input(input: &str) -> Vec<Move> {
    input
        .trim()
        .split(',')
        .map(|m| m.parse().unwrap())
        .collect()
}

#[derive(Debug, PartialEq, Eq)]
struct Move {
    x: i32,
    y: i32,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        let magnitude: i32 = s[1..].parse().unwrap();
        match s.chars().nth(0).unwrap() {
            'U' => Ok(Move { x: 0, y: magnitude }),
            'D' => Ok(Move {
                x: 0,
                y: -magnitude,
            }),
            'L' => Ok(Move {
                x: -magnitude,
                y: 0,
            }),
            'R' => Ok(Move { x: magnitude, y: 0 }),
            _ => Err(()),
        }
    }
}

fn part2(_input: &str) -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input("L54,R20"),
            vec!(Move { x: -54, y: 0 }, Move { x: 20, y: 0 })
        );
    }
}
