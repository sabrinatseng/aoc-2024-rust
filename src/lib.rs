use std::{fmt::Debug, str::FromStr};

pub mod template;

// Parse input string into Vec of Vec of multiple items per line
pub fn parse_from_lines<'a, T>(
    input: &'a str,
) -> impl Iterator<Item = impl Iterator<Item = T> + 'a> + 'a
where
    T: FromStr + 'a,
    T::Err: Debug,
{
    input.lines().map(|line| {
        line.split_whitespace()
            .map(str::parse::<T>)
            .map(|res| res.expect("Failed to parse"))
    })
}
