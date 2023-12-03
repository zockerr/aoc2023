use color_eyre::eyre::{Result};
use crate::day3::{day3part2};

mod day1;
mod day2;
mod day3;

fn main() -> Result<()> {
    color_eyre::install()?;
    day3part2()?;
    Ok(())
}

