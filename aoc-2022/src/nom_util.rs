use anyhow::{anyhow, Result};
use nom::{combinator::all_consuming, error::convert_error, Finish};

pub type IError<'a> = nom::error::VerboseError<&'a str>;
pub type IResult<'a, T> = nom::IResult<&'a str, T, IError<'a>>;

pub fn run_parser<'a, T>(parser: fn(&'a str) -> IResult<T>, input: &'a str) -> Result<T> {
    all_consuming(parser)(input)
        .finish()
        .map(|(_, out)| out)
        .map_err(|e| {
            let msg = convert_error(input, e);
            anyhow!("Parse error: {msg}")
        })
}
