use std::collections::HashSet;
use std::io::stdin;
use color_eyre::eyre::{eyre, Result};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{multispace1};
use nom::{Finish, IResult};
use nom::multi::separated_list0;

struct Card {
    winning_numbers: HashSet<i32>,
    picked_numbers: HashSet<i32>
}

impl Card {
    fn points(&self) -> Result<usize>{
        let length = self.picked_numbers.intersection(&self.winning_numbers).count();
        if length > 0 {
            Ok(2_usize.pow((length - 1) as u32))
        } else {
            Ok(0)
        }
    }
}

#[allow(dead_code)]
pub fn day4part1() -> Result<()> {
    let result = stdin().lines()
        .map(|line| {
            let (_, card)= parse_card(&line?).finish().map_err(|_| eyre!("parsing error"))?;
            Ok(card)
        })
        .map(|card: Result<Card>| card?.points())
        .fold_ok(0, |a, b| a + b)?;
    println!("{result}");
    Ok(())
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, _) = tag("Card")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = nom::character::complete::i32(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, winning_numbers) = separated_list0(
        multispace1,
        nom::character::complete::i32
    )(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("|")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, picked_numbers) = separated_list0(
        multispace1,
        nom::character::complete::i32
    )(input)?;
    Ok((input, Card{picked_numbers: HashSet::from_iter(picked_numbers), winning_numbers: HashSet::from_iter(winning_numbers)}))
}