#![allow(dead_code)]

use nom::{IResult, branch::alt, error::{VerboseError, context}, multi::many0, combinator::map};
use types::TextParts;
use wasm_bindgen::prelude::*;

pub mod types;
mod parse_uri;
mod parse_mention;

type Result<T, U> = IResult<T, U, VerboseError<T>>;

#[wasm_bindgen]
pub fn parse_demo(input: &str) -> String {
    context(
        "parse_demo",
        many0(alt((
            map(parse_mention::mention, |res| TextParts::Mention(res)),
            map(parse_uri::uri, |res| TextParts::URI(res)),
        )))
    )(input)
        .map(|(next_input, res)| {
            dbg!(&res);
            (next_input, res)
        });
    String::new()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_demo() {
        assert_eq!(crate::parse_demo("").as_str(), "")
    }
}
