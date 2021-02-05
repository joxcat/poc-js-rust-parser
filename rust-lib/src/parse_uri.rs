use nom::{AsChar, InputTakeAtPosition, branch::alt, bytes::complete::{tag, tag_no_case, take, take_till}, character::{complete::{alpha1, one_of}, is_digit}, error::VerboseError, error::{ErrorKind, context}, multi::{count, many1, many_m_n}, sequence::{preceded, terminated, tuple}};
use nom::Err as NomErr;

use crate::{Result, types::{Host, IPNum, Scheme}};

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
            tuple((many_m_n(1, 1, alphanumerichyphen1), take(0_usize))),
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
    context(
        "ip",
        alt((
            tuple((count(terminated(ipv4_num, tag(".")), 3), ipv4_num)),
            tuple((preceded(tag("["), many1(terminated(ipv6_num, tag(":")))), terminated(ipv6_num, tag("]")))),
        )),
    )(input)
        .map(|(next_input, res)| {
            match res.1 {
                IPNum::IPV4(n) => {
                    let mut ip: [u8; 4] = [0,0,0,0];
                    res.0
                       .into_iter()
                       .enumerate()
                       .for_each(|(idx, value)| {
                           let value: Option<u8> = value.into();
                           ip[idx] = value.unwrap();
                       });
                    ip[3] = n;
                    (next_input, Host::IPV4(ip))
                },
                IPNum::IPV6(n) => {
                    let mut ip: [u16; 8] = [0,0,0,0,0,0,0,0];
                    let mut ip_iter = res.0
                       .into_iter()
                       .collect::<Vec<IPNum>>();
                    ip_iter.reverse();

                    ip_iter.into_iter()
                           .enumerate()
                           .for_each(|(idx, value)| {
                        let value: Option<u16> = value.into();
                        ip[6 - idx] = value.unwrap();
                    });

                    ip[7] = n;
                    (next_input, Host::IPV6(ip))
                }
            }
        })
}

fn ipv4_num(input: &str) -> Result<&str, IPNum> {
    context("ipv4 number", n_to_m_digits(1, 3))(input).and_then(|(next_input, res)| {
        match res.parse::<u8>() {
            Ok(n) => Ok((next_input, IPNum::IPV4(n))),
            Err(_) => Err(NomErr::Error(VerboseError { errors: vec![] })),
        }
    })
}

fn ipv6_num(input: &str) -> Result<&str, IPNum> {
    context("ipv6 number", n_to_m_hexas(0, 4))(input).and_then(|(next_input, res)| {
        if res.is_empty() {
            Ok((next_input, IPNum::IPV6(0)))
        } else {
            match u16::from_str_radix(&res, 16) {
                Ok(n) => Ok((next_input, IPNum::IPV6(n))),
                Err(_) => Err(NomErr::Error(VerboseError { errors: vec![] })),
            }
        }
    })
}

fn ip_or_host(input: &str) -> Result<&str, Host> {
    context("ip or domain", alt((ip, host)))(input)
}

fn port(input: &str) -> Result<&str, u16> {
    context("port", take_till(is_digit))(input)
        .map(|(next_input, res)| {
            dbg!(res);

            Ok((next_input, res.parse::<u16>))
        })
}

// utils
fn alphanumerichyphen1<T>(input: T) -> Result<T, T>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    input.split_at_position1_complete(
        |item| {
            let char_item = item.as_char();
            char_item != '-' && !char_item.is_alphanum()
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

    #[test]
    fn test_ip() {
        assert_eq!(ip("127.0.0.1:8080"), Ok((":8080", Host::IPV4([127,0,0,1]))));
        assert_eq!(ip("0.0.0.0"), Ok(("", Host::IPV4([0,0,0,0]))));
        assert_eq!(ip("[0:0:0:0:0:0:0:1]"), Ok(("", Host::IPV6([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x1]))));
        assert_eq!(ip("[::1]"), Ok(("", Host::IPV6([0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x1]))));
        assert_eq!(ip("[2a01:5cc0:1:2::4]:8080"), Ok((":8080", Host::IPV6([0x0,0x0,0x2a01,0x5cc0,0x1,0x2,0x0,0x4]))));
    }

    #[test]
    fn test_ip_or_host() {
        assert_eq!(ip_or_host("[::ffff:124:1234:124]:4242"), Ok((":4242", Host::IPV6([0x0,0x0,0x0,0x0,0xffff,0x124,0x1234,0x124]))));
        assert_eq!(ip_or_host("example.org:69420"), Ok((":69420", Host::HOST("example.org".to_string()))));
    }
}
