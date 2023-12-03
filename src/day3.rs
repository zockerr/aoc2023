use std::io::stdin;
use color_eyre::eyre::{Context, eyre, Result};
use itertools::Itertools;
#[allow(dead_code)]
pub fn day3part1() -> Result<()> {
    //load the input for easier parsing
    let data: Vec<Vec<char>> = stdin().lines().map::<Result<_>,_>(|line| Ok(line.wrap_err("error getting line")?.chars().collect())).try_collect()?;

    let mut valid_numbers: Vec<u32> = Vec::new();
    //iterate over lines, parse and filter numbers
    for i in 0..data.len() {
        let mut number_currenty_parsing: u32 = 0;
        let mut symbol_found: bool = false;
        for j in 0..data[i].len() {
            if data[i][j].is_numeric() {
                //Wir haben eine Ziffer
                if !symbol_found {
                    //prüfen, ob ein Symbol in der akuten Nähe
                    symbol_found = search_symbol(&data, i, j);
                }
                number_currenty_parsing = number_currenty_parsing*10 + data[i][j].to_digit(10).ok_or(eyre!("Error parsing digit"))?;
            } else {
                //Wir haben das Ende einer validen Zahl erreicht
                if symbol_found {
                    valid_numbers.push(number_currenty_parsing);
                }
                number_currenty_parsing = 0;
                symbol_found = false;
            }
        }
        if symbol_found {
            //Wir haben das Ende einer Zeile erreicht
            valid_numbers.push(number_currenty_parsing);
        }
    }

    println!("{}", valid_numbers.iter().sum::<u32>());

    Ok(())
}
#[allow(dead_code)]
pub fn day3part2() -> Result<()> {
    //load the input for easier parsing
    let data: Vec<Vec<char>> = stdin().lines().map::<Result<_>,_>(|line| Ok(line.wrap_err("error getting line")?.chars().collect())).try_collect()?;

    let mut ratios: Vec<u64> = Vec::new();
    //iterate over lines, parse and filter numbers
    for i in 0..data.len() {
        for j in 0..data[i].len() {
            if data[i][j] == '*' {
                let i1 = calc_ratio(&data, i, j) as u64;
                if i1 > 0 {
                    ratios.push(i1)
                }
            }
        }
    }

    println!("{}", ratios.iter().sum::<u64>());

    Ok(())
}

fn calc_ratio(data: &Vec<Vec<char>>, i: usize, j: usize) -> u32 {
    let mut parsed_numbers: Vec<u32> = Vec::new();
    //Obere Zeile
    if i > 0 {
        if data[i-1][j].is_numeric() {
            parsed_numbers.push(parse_number_from(data, i-1, j));
        } else {
            if j > 0 && data[i-1][j-1].is_numeric() {
                parsed_numbers.push(parse_number_from(data, i-1, j-1));
            }
            if j < data[i-1].len()-1 && data[i-1][j+1].is_numeric() {
                parsed_numbers.push(parse_number_from(data, i-1, j+1));
            }
        }
    }
    //Mittlere Zeile
    if j > 0 && data[i][j-1].is_numeric() {
        parsed_numbers.push(parse_number_from(data, i, j-1));
    }
    if j < data[i].len()-1 && data[i][j+1].is_numeric() {
        parsed_numbers.push(parse_number_from(data, i, j+1));
    }
    //Untere Zeile
    if i < data.len()-1 {
        if data[i+1][j].is_numeric() {
            parsed_numbers.push(parse_number_from(data, i+1, j));
        } else {
            if j > 0 && data[i+1][j-1].is_numeric() {
                parsed_numbers.push(parse_number_from(data, i+1, j-1));
            }
            if j < data[i+1].len()-1 && data[i+1][j+1].is_numeric() {
                parsed_numbers.push(parse_number_from(data, i+1, j+1));
            }
        }
    }
    if parsed_numbers.len() == 2 {
        parsed_numbers[0]*parsed_numbers[1]
    } else {
        0
    }
}

fn parse_number_from(data: &Vec<Vec<char>>, i: usize, j: usize) -> u32 {
    let mut c = j;
    //erste Ziffer suchen
    while c > 0 && data[i][c-1].is_numeric() {
        c -= 1;
    }
    let mut result: u32 = 0;
    while c < data[i].len() && data[i][c].is_numeric() {
        result = result*10 + data[i][c].to_digit(10).unwrap();
        c += 1;
    }
    result
}

fn search_symbol(data: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let mut found = false;
    if i > 0 {
        // i-1
        if j > 0 {
            found = found || (data[i-1][j-1] != '.' && !data[i-1][j-1].is_numeric());
        }
        found = found || (data[i-1][j] != '.' && !data[i-1][j].is_numeric());
        if j < data[i].len()-1 {
            found = found || (data[i-1][j+1] != '.' && !data[i-1][j+1].is_numeric());
        }
    }
    if j > 0 {
        found = found || (data[i][j-1] != '.' && !data[i][j-1].is_numeric());
    }
    if j < data[i].len()-1 {
        found = found || (data[i][j+1] != '.' && !data[i][j+1].is_numeric());
    }
    if i < data.len()-1 {
        if j > 0 {
            found = found || (data[i+1][j-1] != '.' && !data[i+1][j-1].is_numeric());
        }
        found = found || (data[i+1][j] != '.' && !data[i+1][j].is_numeric());
        if j < data[i].len()-1 {
            found = found || (data[i+1][j+1] != '.' && !data[i+1][j+1].is_numeric());
        }
    }
    found
}