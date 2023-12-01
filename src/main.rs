use color_eyre::eyre::{Result};
use crate::day1::day1part1;

mod day1;

fn main() -> Result<()> {
    color_eyre::install()?;
    day1part1()?;
    Ok(())
}

