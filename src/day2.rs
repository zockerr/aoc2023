use std::io::stdin;
use color_eyre::eyre::{eyre, WrapErr, Result};
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::character::complete::u32;
use nom::{Finish, IResult};
use nom::multi::{separated_list0};
use nom::sequence::{separated_pair};


#[allow(dead_code)]
pub fn day2part1() -> color_eyre::Result<()> {
    let result: i32 = stdin().lines().map(|line| {
        let (_, game)= parse_game(&line?).finish().map_err(|_| eyre!("parsing error"))?;
        let value:Result<i32> = Ok(if game.is_possible() { game.id } else { 0 });
        value
    })
        .fold_ok(0, |a, b| a + b)?;
    println!("{result}");
    Ok(())
}

#[derive(Eq, PartialEq, Debug)]
struct Game {
    id: i32,
    rounds: Vec<GameRound>
}

impl Game {
    fn is_possible(&self) -> bool {
        self.rounds.iter().all(GameRound::is_possible)
    }
}

#[derive(Eq, PartialEq, Debug)]
struct GameRound {
    red: i32,
    green: i32,
    blue: i32
}

impl GameRound {
    fn is_possible(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}

fn parse_game_round(input: &str) -> IResult<&str, GameRound> {
    let (input, elements): (&str, Vec<(u32, &str)>) = separated_list0(
        tag(", "),
        alt(
            (
                separated_pair(u32, char(' '), tag("blue")),
                separated_pair(u32, char(' '), tag("red")),
                separated_pair(u32, char(' '), tag("green")),
            )
        )
    )(input)?;
    let mut result = GameRound{
        red: 0,
        green: 0,
        blue: 0,
    };
    for (value, color) in elements {
        match color {
            "blue" => result.blue = value as i32,
            "red" => result.red = value as i32,
            "green" => result.green = value as i32,
            _ => panic!("invalid color") //This should never happen, because the parser doesn't allow it
        }
    }
    Ok((input, result))
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = nom::character::complete::i32(input)?;
    let (input, _) = tag(": ")(input)?;
    let (_, rounds) = separated_list0(
        tag("; "),
        parse_game_round
    )(input)?;
    Ok(("", Game{id, rounds}))
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn round_possible_test() {
        assert_eq!(GameRound { red: 0, green: 0, blue: 0 }.is_possible(), true);
        assert_eq!(GameRound { red: 12, green: 13, blue: 14 }.is_possible(), true);
        assert_eq!(GameRound { red: 13, green: 13, blue: 14 }.is_possible(), false);
        assert_eq!(GameRound { red: 12, green: 14, blue: 14 }.is_possible(), false);
        assert_eq!(GameRound { red: 12, green: 13, blue: 15 }.is_possible(), false);
    }

    #[test]
    fn parse_game_round_test() {
        assert!(matches!(parse_game_round(""), Ok((_, GameRound{ red: 0, green: 0, blue: 0 }))));
        assert!(matches!(parse_game_round("3 blue"), Ok((_, GameRound{ red: 0, green: 0, blue: 3 }))));
        assert!(matches!(parse_game_round("3 red"), Ok((_, GameRound{ red: 3, green: 0, blue: 0 }))));
        assert!(matches!(parse_game_round("3 green"), Ok((_, GameRound{ red: 0, green: 3, blue: 0 }))));
        assert!(matches!(parse_game_round("3 green, 12 blue"), Ok((_, GameRound{ red: 0, green: 3, blue: 12 }))));
        assert!(matches!(parse_game_round("3 green, 12 blue, 22 red"), Ok((_, GameRound{ red: 22, green: 3, blue: 12 }))));
    }

    #[test]
    fn parse_game_test() {
        assert_eq!(
            Game{
                id: 1,
                rounds: vec![
                    GameRound{blue: 3, red: 4, green: 0},
                    GameRound{blue: 6, red: 1, green: 2},
                    GameRound{blue: 0, red: 0, green: 2}]},
            parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").unwrap().1
        );
        assert_eq!(
            Game{
                id: 2,
                rounds: vec![
                    GameRound{blue: 1, red: 0, green: 2},
                    GameRound{blue: 4, red: 1, green: 3},
                    GameRound{blue: 1, red: 0, green: 1}]},
            parse_game("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue").unwrap().1
        );
    }
}