mod error;

use dsv::Parser;
use error::{Result, SpeakError};
use std::{fs::File, num::ParseIntError, path::PathBuf, result, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
pub struct Name {
    first: String,
    last: String,
}

impl Name {
    pub fn new(first: &str, last: &str) -> Name {
        Name {
            first: String::from(first),
            last: String::from(last),
        }
    }
}

impl FromStr for Name {
    type Err = SpeakError;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        Ok(match parts.as_slice() {
            [first] => Name::new(first, ""),
            [first @ .., last] => Name::new(&first.join(" "), last),
            _ => Name::new("", ""),
        })
    }
}

#[derive(clap::Parser, Debug)]
pub struct Args {
    pub history_file: PathBuf,
}

pub fn list_speakers(args: &Args) -> Result<()> {
    let history_file = File::open(&args.history_file)?;
    let rows = Parser::new().parse(history_file);
    for row in rows {
        println!("{:?}", row);
    }

    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
pub struct Day {
    year: u32,
    month: u32,
    day: u32,
}

impl Day {
    fn new(year: u32, month: u32, day: u32) -> Day {
        Day {
            year: year,
            month: month,
            day: day,
        }
    }
}

impl FromStr for Day {
    type Err = SpeakError;

    fn from_str(s: &str) -> Result<Self> {
        let parts: result::Result<Vec<u32>, ParseIntError> =
            s.split('/').map(|part| part.parse()).collect();

        match parts
            .map_err(|_| SpeakError::Date(s.to_string()))?
            .as_slice()
        {
            &[month, day, year] => Ok(Day::new(year, month, day)),
            _ => Err(SpeakError::Date(s.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_name() {
        assert_eq!(
            Ok(Name::new("Fred", "Flintstone")),
            "Fred Flintstone".parse()
        );
    }

    #[test]
    fn parse_three_part_name() {
        assert_eq!(
            Ok(Name::new("Fred Fredington", "Flintstone")),
            "Fred Fredington Flintstone".parse()
        );
    }

    #[test]
    fn parse_single_name() {
        assert_eq!(Ok(Name::new("Fred", "")), "Fred".parse());
    }

    #[test]
    fn parse_day() {
        assert_eq!(Ok(Day::new(2022, 12, 1)), "12/1/2022".parse());
    }

    #[test]
    fn parse_malformed_day() {
        assert!("blay".parse::<Day>().is_err());
    }

    #[test]
    fn parse_leading_zeroes_day() {
        assert_eq!(Ok(Day::new(2053, 7, 8)), "07/08/2053".parse());
    }
}
