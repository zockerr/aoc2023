use std::io::stdin;
use color_eyre::eyre::{eyre, Result};
use itertools::Itertools;

pub fn day1part1() -> Result<()> {
    let result: i32 = stdin().lines().map(|line| parse_line(&line?)).fold_ok(0, |a, b| a + b)?;
    println!("{result}");
    Ok(())
}

fn parse_line(line: &str) -> Result<i32> {
    let number_string:Vec<char> = line.chars().filter(|char| char.is_numeric())
        .collect();
    let mut value_string = String::new();
    value_string.push(*number_string.first().ok_or(eyre!("No digits in String!"))?);
    value_string.push(*number_string.last().ok_or(eyre!("No digits in String!"))?);
    value_string.parse::<i32>().map_err(|_|eyre!("Could not parse!"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_test() {
        assert!(matches!(parse_line("a1b2c3d4e5f"), Ok(15)));
        assert!(matches!(parse_line("pqr3stu8vwx"), Ok(38)));
        assert!(matches!(parse_line("1abc2"), Ok(12)));
        assert!(matches!(parse_line("treb7uchet"), Ok(77)));
    }
}