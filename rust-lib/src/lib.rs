#![allow(dead_code)]

use nom::{IResult, error::VerboseError};
use wasm_bindgen::prelude::*;

pub mod types;
mod parse_uri;

type Result<T, U> = IResult<T, U, VerboseError<T>>;

#[wasm_bindgen]
pub fn parse_demo(text: &str) -> String {
    return String::new();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_demo() {
        assert_eq!(crate::parse_demo("").as_str(), "")
    }
}
