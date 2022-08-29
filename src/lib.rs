use std::{num::ParseIntError, result};

type Result<T> = result::Result<T, SpeakError>;

#[derive(Debug, PartialEq, Eq)]
struct SpeakError {
    message: String,
}

impl From<ParseIntError> for SpeakError {
    fn from(e: ParseIntError) -> Self {
        SpeakError {
            message: format!("ParseIntError: {}", e.to_string()),
        }
    }
}

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

    pub fn parse(s: &str) -> Name {
        let parts: Vec<&str> = s.split_whitespace().collect();
        match parts.as_slice() {
            [first] => Name::new(first, ""),
            [first @ .., last] => Name::new(&first.join(" "), last),
            _ => Name::new("", ""),
        }
    }
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

    fn parse(s: &str) -> Result<Day> {
        let parts: result::Result<Vec<u32>, ParseIntError> =
            s.split('/').map(|part| part.parse()).collect();

        match parts?.as_slice() {
            &[month, day, year] => Ok(Day::new(year, month, day)),
            _ => Err(SpeakError {
                message: format!("Problem parsing `{}' as a Day", s),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_name() {
        assert_eq!(
            Name::new("Fred", "Flintstone"),
            Name::parse("Fred Flintstone")
        );
    }

    #[test]
    fn parse_three_part_name() {
        assert_eq!(
            Name::new("Fred Fredington", "Flintstone"),
            Name::parse("Fred Fredington Flintstone")
        );
    }

    #[test]
    fn parse_single_name() {
        assert_eq!(Name::new("Fred", ""), Name::parse("Fred"));
    }

    #[test]
    fn parse_day() {
        assert_eq!(Ok(Day::new(2022, 12, 1)), Day::parse("12/1/2022"));
    }

    #[test]
    fn parse_malformed_day() {
        assert!(Day::parse("blay").is_err());
    }

    #[test]
    fn parse_leading_zeroes_day() {
        assert_eq!(Ok(Day::new(2053, 7, 8)), Day::parse("07/08/2053"));
    }
}
