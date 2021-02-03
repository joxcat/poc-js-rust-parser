use nom::{AsChar, InputTakeAtPosition, error::VerboseError, branch::alt, bytes::complete::tag, bytes::complete::tag_no_case, bytes::complete::take, character::complete::alpha1, character::complete::one_of, error::{ErrorKind, context}, multi::{many1, many_m_n}, sequence::{terminated, tuple}};
use nom::Err as NomErr;

use crate::{Result, types::{Host, Scheme}};

fn scheme(input: &str) -> Result<&str, Scheme> {
    context(
        "scheme",
        alt((tag_no_case("HTTP://"), tag_no_case("HTTPS://"))),
    )(input)
        .map(|(next_input, res)| (next_input, res.into()))
}

fn host(input: &str) -> Result<&str, Host> {
    context(
        "host",
        alt((
            tuple((many1(terminated(alphanumerichyphen1, tag("."))), alpha1)),
            tuple((many_m_n(1, 1, alphanumerichyphen1), take(0 as usize))),
        )),
    )(input)
        .map(|(next_input, mut res)| {
            if !res.1.is_empty() {
                res.0.push(res.1);
            }
            (next_input, Host::HOST(res.0.join(".")))
        })
}

fn ip(input: &str) -> Result<&str, Host> {
    unimplemented!("https://blog.logrocket.com/parsing-in-rust-with-nom/#parsingurlswithauthority")
    // context(
    //     "ip",
    // )
}

fn ipv4_num(input: &str) -> Result<&str, u8> {
    context("ipv4 number", n_to_m_digits(1, 3))(input).and_then(|(next_input, res)| {
        match res.parse::<u8>() {
            Ok(n) => Ok((next_input, n)),
            Err(_) => Err(NomErr::Error(VerboseError { errors: vec![] })),
        }
    })
}

fn ipv6_num(input: &str) -> Result<&str, u16> {
    context("ipv6 number", n_to_m_hexas(0, 4))(input).and_then(|(next_input, res)| {
        match u16::from_str_radix(&res, 16) {
            Ok(n) => Ok((next_input, n)),
            Err(_) => Err(NomErr::Error(VerboseError { errors: vec![] })),
        }
    })
}

// Utils
fn alphanumerichyphen1<T>(input: T) -> Result<T, T>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    input.split_at_position1_complete(
        |item| {
            let char_item = item.as_char();
            !(char_item == '-') && !char_item.is_alphanum()
        },
        ErrorKind::AlphaNumeric,
    )
}

fn n_to_m_digits<'a>(n: usize, m: usize) -> impl FnMut(&'a str) -> Result<&str, String> {
    move |input| {
        many_m_n(n, m, one_of("0123456789"))(input)
            .map(|(next_input, res)| (next_input, res.into_iter().collect()))
    }
}

fn n_to_m_hexas<'a>(n: usize, m: usize) -> impl FnMut(&'a str) -> Result<&str, String> {
    move |input| {
        many_m_n(n, m, one_of("0123456789abcdef"))(input)
            .map(|(next_input, res)| (next_input, res.into_iter().collect()))
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheme() {
        assert_eq!(scheme("https://yay"), Ok(("yay", Scheme::HTTPS)));
        assert_eq!(scheme("http://yay"), Ok(("yay", Scheme::HTTP)));
    }

    #[test]
    fn test_host() {
        assert_eq!(host("localhost:8080"), Ok((":8080", Host::HOST("localhost".to_string()))));
        assert_eq!(host("example.org:8080"), Ok((":8080", Host::HOST("example.org".to_string()))));
        assert_eq!(host("sub.example.org:8080"), Ok((":8080", Host::HOST("sub.example.org".to_string()))));
        assert_eq!(host("example.123"), Ok((".123", Host::HOST("example".to_string()))));
    }
}
