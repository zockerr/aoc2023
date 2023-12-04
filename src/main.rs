use color_eyre::eyre::{Result};
use crate::day4::day4part1;

mod day1;
mod day2;
mod day3;
mod day4;

fn main() -> Result<()> {
    color_eyre::install()?;
    day4part1()?;
    Ok(())
}

