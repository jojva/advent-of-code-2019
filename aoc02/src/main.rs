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
    let mut values: Vec<usize> = input
        .trim()
        .split(',')
        .map(|v| v.parse().unwrap())
        .collect();
    run_intcode(&mut values);
    writeln!(io::stdout(), "{}", values[0])?;
    Ok(())
}

fn part2(_input: &str) -> Result<()> {
    Ok(())
}

fn run_intcode(values: &mut Vec<usize>) -> &Vec<usize> {
    for i in (0..values.len()).step_by(4) {
        match values[i] {
            1 => {
                let pos_operator1 = values[i + 1];
                let pos_operator2 = values[i + 2];
                let pos_result = values[i + 3];
                values[pos_result] = values[pos_operator1] + values[pos_operator2];
            }
            2 => {
                let pos_operator1 = values[i + 1];
                let pos_operator2 = values[i + 2];
                let pos_result = values[i + 3];
                values[pos_result] = values[pos_operator1] * values[pos_operator2];
            }
            99 => break,
            _ => panic!(),
        }
    }
    values
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_intcode() {
        assert_eq!(
            run_intcode(&mut vec!(1, 0, 0, 0, 99)),
            &vec!(2, 0, 0, 0, 99)
        );
        assert_eq!(
            run_intcode(&mut vec!(2, 3, 0, 3, 99)),
            &vec!(2, 3, 0, 6, 99)
        );
        assert_eq!(
            run_intcode(&mut vec!(2, 4, 4, 5, 99, 0)),
            &vec!(2, 4, 4, 5, 99, 9801)
        );
        assert_eq!(
            run_intcode(&mut vec!(1, 1, 1, 4, 99, 5, 6, 0, 99)),
            &vec!(30, 1, 1, 4, 2, 5, 6, 0, 99)
        );
    }
}
