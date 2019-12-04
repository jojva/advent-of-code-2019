extern crate nalgebra as na;

use na::{Point2, Vector2};
use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

struct Segment {
    p0: Point2<i32>,
    p1: Point2<i32>,
}

impl Segment {
    fn zero() -> Self {
        Self {
            p0: Point2::new(0, 0),
            p1: Point2::new(0, 0),
        }
    }

    fn intersects(rhs: &Segment) -> bool {

    }

    fn intersection_points(rhs: &Segment) -> Vec<Point2> {

    }
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let moves_1 = parse_input(input.lines().nth(0).unwrap());
    let moves_2 = parse_input(input.lines().nth(1).unwrap());
    let mut segment_1 = Segment::zero();
    for move_1 in &moves_1 {
        let mut segment_2 = Segment::zero();
        for move_2 in &moves_2 {

        }
    }
    writeln!(io::stdout(), "{:?}", moves_1)?;
    Ok(())
}

fn part2(_input: &str) -> Result<()> {
    Ok(())
}

fn parse_input(input: &str) -> Vec<Vector2<i32>> {
    input
        .trim()
        .split(',')
        .map(|movement| {
            let magnitude: i32 = movement[1..].parse().unwrap();
            match movement.chars().nth(0).unwrap() {
                'U' => Vector2::new(0, magnitude),
                'D' => Vector2::new(0, -magnitude),
                'L' => Vector2::new(-magnitude, 0),
                'R' => Vector2::new(magnitude, 0),
                _ => panic!(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input("L54,R20"),
            vec!(Vector2::new(-54, 0), Vector2::new(20, 0))
        );
    }
}
