use color_eyre::eyre::{Result};
use crate::day2::{day2part2};

mod day1;
mod day2;

fn main() -> Result<()> {
    color_eyre::install()?;
    day2part2()?;
    Ok(())
}

