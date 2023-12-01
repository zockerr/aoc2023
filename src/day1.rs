use std::io::stdin;
use color_eyre::eyre::{eyre, Result};
use color_eyre::owo_colors::OwoColorize;
use itertools::Itertools;

#[allow(dead_code)]
pub fn day1part1() -> Result<()> {
    let result: i32 = stdin().lines().map(|line| parse_line(&line?)).fold_ok(0, |a, b| a + b)?;
    println!("{result}");
    Ok(())
}

pub fn day1part2() -> Result<()> {
    let result: i32 = stdin().lines()
        .map(|line| {
            let l = line.map_err(|_|eyre!("Couldn't get line"))?;
            let mut result = String::new();
            result.push(find_first_digit(&l).map_err(|_|eyre!("Couldn't find first digit"))?);
            result.push(find_last_digit(&l).map_err(|_|eyre!("Couldn't find second digit"))?);
            result.parse::<i32>().map_err(|_|eyre!("Couldn't parse number"))
        })
        .fold_ok(0, |a, b| a + b)?;
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

fn replace_number_words(input: &str) -> Result<String> {
    let mut current = 0;
    let mut result = String::new();
    while current < input.len() {
        let s = &input[current..input.len()];
        if(s).starts_with("one") {result.push('1'); current += 3;}
        else if(s).starts_with("two") {result.push('2'); current += 3;}
        else if(s).starts_with("three") {result.push('3'); current += 5;}
        else if(s).starts_with("four") {result.push('4'); current += 4;}
        else if(s).starts_with("five") {result.push('5'); current += 4;}
        else if(s).starts_with("six") {result.push('6'); current += 3;}
        else if(s).starts_with("seven") {result.push('7'); current += 5;}
        else if(s).starts_with("eight") {result.push('8'); current += 5;}
        else if(s).starts_with("nine") {result.push('9'); current += 4;}
        else {result.push(s.chars().next().ok_or(eyre!("wat."))?); current += 1;}
    }
    Ok(result)
}

fn find_first_digit(input: &str) -> Result<char> {
    let mut current = 0;
    while current < input.len() {
        let s = &input[current..input.len()];
        if(s).starts_with("one") {return Ok('1')}
        else if(s).starts_with("two") {return Ok('2')}
        else if(s).starts_with("three") {return Ok('3')}
        else if(s).starts_with("four") {return Ok('4')}
        else if(s).starts_with("five") {return Ok('5')}
        else if(s).starts_with("six") {return Ok('6')}
        else if(s).starts_with("seven") {return Ok('7')}
        else if(s).starts_with("eight") {return Ok('8')}
        else if(s).starts_with("nine") {return Ok('9')}
        else if s.chars().next().ok_or(eyre!("wat."))?.is_numeric() {return s.chars().next().ok_or(eyre!("wat."))}
        else {current += 1}
    }
    Err(eyre!("No digit found!"))
}

fn find_last_digit(input: &str) -> Result<char> {
    let mut current = input.len();
    while current > 0 {
        let s = &input[0..current];
        if(s).ends_with("one") {return Ok('1')}
        else if(s).ends_with("two") {return Ok('2')}
        else if(s).ends_with("three") {return Ok('3')}
        else if(s).ends_with("four") {return Ok('4')}
        else if(s).ends_with("five") {return Ok('5')}
        else if(s).ends_with("six") {return Ok('6')}
        else if(s).ends_with("seven") {return Ok('7')}
        else if(s).ends_with("eight") {return Ok('8')}
        else if(s).ends_with("nine") {return Ok('9')}
        else if s.chars().next_back().ok_or(eyre!("wat."))?.is_numeric() {return s.chars().next_back().ok_or(eyre!("wat."))}
        else {current -= 1}
    }
    Err(eyre!("No digit found!"))
}



#[cfg(test)]
mod tests {
    use super::*;
    use color_eyre::eyre::{Result};

    #[test]
    fn parse_line_test() {
        assert!(matches!(parse_line("a1b2c3d4e5f"), Ok(15)));
        assert!(matches!(parse_line("pqr3stu8vwx"), Ok(38)));
        assert!(matches!(parse_line("1abc2"), Ok(12)));
        assert!(matches!(parse_line("treb7uchet"), Ok(77)));
    }

    #[test]
    fn replace_number_words_test() -> Result<()> {
        assert_eq!(replace_number_words("two1nine")?, "219");
        assert_eq!(replace_number_words("eightwothree")?, "8wo3");
        assert_eq!(replace_number_words("abcone2threexyz")?, "abc123xyz");
        assert_eq!(replace_number_words("xtwone3four")?, "x2ne34");
        assert_eq!(replace_number_words("4nineeightseven2")?, "49872");
        assert_eq!(replace_number_words("zoneight234")?, "z1ight234");
        assert_eq!(replace_number_words("eighttkbtzjz6nineeight")?, "8tkbtzjz698");
        assert_eq!(replace_number_words("5knjbxgvhktvfcq89onefive")?, "5knjbxgvhktvfcq8915");
        assert_eq!(replace_number_words("hnjcrxeightonejnlvm4hstmcsevensix")?, "hnjcrx81jnlvm4hstmc76");
        assert_eq!(replace_number_words("trsdgcxcseven39dpmzs")?, "trsdgcxc739dpmzs");
        assert_eq!(replace_number_words("oneninesixtwo26")?, "196226");
        assert_eq!(replace_number_words("dppthreeh32twobhrqzks")?, "dpp3h322bhrqzks");
        assert_eq!(replace_number_words("1cxklgfbvhsnccfive4")?, "1cxklgfbvhsncc54");
        assert_eq!(replace_number_words("foursgjsevenseven5five19")?, "4sgj775519");
        assert_eq!(replace_number_words("nrrk87")?, "nrrk87");
        assert_eq!(replace_number_words("63ntkjbvcv3ntdcptmvheight78")?, "63ntkjbvcv3ntdcptmvh878");
        assert_eq!(replace_number_words("7xv3one")?, "7xv31");
        Ok(())
    }

    #[test]
    fn first_digit_test() -> Result<()> {
        assert_eq!(find_first_digit("two1nine")?, '2');
        assert_eq!(find_first_digit("eightwothree")?, '8');
        assert_eq!(find_first_digit("abcone2threexyz")?, '1');
        assert_eq!(find_first_digit("xtwone3four")?, '2');
        assert_eq!(find_first_digit("4nineeightseven2")?, '4');
        assert_eq!(find_first_digit("zoneight234")?, '1');
        assert_eq!(find_first_digit("7pqrstsixteen")?, '7');
        assert_eq!(find_first_digit("eighttkbtzjz6nineeight")?, '8');
        assert_eq!(find_first_digit("5knjbxgvhktvfcq89onefive")?, '5');
        assert_eq!(find_first_digit("hnjcrxeightonejnlvm4hstmcsevensix")?, '8');
        assert_eq!(find_first_digit("trsdgcxcseven39dpmzs")?, '7');
        assert_eq!(find_first_digit("1eighthree")?, '1');

        Ok(())
    }

    #[test]
    fn last_digit_test() -> Result<()> {
        assert_eq!(find_last_digit("two1nine")?, '9');
        assert_eq!(find_last_digit("eightwothree")?, '3');
        assert_eq!(find_last_digit("abcone2threexyz")?, '3');
        assert_eq!(find_last_digit("xtwone3four")?, '4');
        assert_eq!(find_last_digit("4nineeightseven2")?, '2');
        assert_eq!(find_last_digit("zoneight234")?, '4');
        assert_eq!(find_last_digit("7pqrstsixteen")?, '6');
        assert_eq!(find_last_digit("eighttkbtzjz6nineeight")?, '8');
        assert_eq!(find_last_digit("5knjbxgvhktvfcq89onefive")?, '5');
        assert_eq!(find_last_digit("hnjcrxeightonejnlvm4hstmcsevensix")?, '6');
        assert_eq!(find_last_digit("trsdgcxcseven39dpmzs")?, '9');
        assert_eq!(find_last_digit("1eighthree")?, '3');

        Ok(())
    }

}