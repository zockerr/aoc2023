use color_eyre::eyre::{Result};
use crate::day1::{day1part2};

mod day1;

fn main() -> Result<()> {
    color_eyre::install()?;
    day1part2()?;
    Ok(())
}

