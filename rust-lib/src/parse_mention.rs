use nom::{bytes::complete::is_not, character::complete::char, error::{VerboseError, context}, sequence::{preceded, delimited, tuple}};
use nom::Err as NomErr;

use crate::{types::Mention, Result};


pub fn mention(input: &str) -> Result<&str, Mention> {
    context("mention", preceded(char('@'), tuple((
        name,
        user_id,
    ))))(input)
        .map(|(next_input, res)| {
            let (name, user_id) = res;
            (
                next_input,
                Mention {
                    user_id,
                    name,
                }
            )
        })
}

fn user_id(input: &str) -> Result<&str, u32> {
    context("user_id", delimited(char('('), is_not(")"), char(')')))(input)
        .and_then(|(next_input, res)| {
            match res.parse::<u32>() {
                Ok(n) => Ok((next_input, n)),
                Err(_) => Err(NomErr::Error(VerboseError { errors: vec![] })),
            }
        })
}

fn name(input: &str) -> Result<&str, &str> {
    context("name", delimited(char('['), is_not("]"), char(']')))(input)
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mention() {
        assert_eq!(mention("@[helloworld](0)"), Ok(("", Mention {
            user_id: 0_u32,
            name: "helloworld",
        })));
    }

    #[test]
    fn test_user_id() {
       assert_eq!(user_id("(42)"), Ok(("", 42_u32)));
    }

    #[test]
    fn test_name() {
        assert_eq!(name("[example](123)"), Ok(("(123)", "example")));
    }
}
