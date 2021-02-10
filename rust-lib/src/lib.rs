use nom::{IResult, branch::alt, combinator::map, bytes::complete::take, error::{VerboseError, context}, multi::many0};
use types::{Host, Scheme, TextParts};
use wasm_bindgen::prelude::*;

pub mod types;
pub mod parse_uri;
pub mod parse_mention;

type Result<T, U> = IResult<T, U, VerboseError<T>>;

pub fn rust_parse_demo(input: &str) -> Vec<TextParts> {
    context(
        "parse_demo",
        many0(alt((
            map(parse_mention::mention, |res| TextParts::Mention(res)),
            map(parse_uri::uri, |res| TextParts::URI(res)),
            map(take(1_usize), |res| TextParts::Other(res)),
        )))
    )(input).expect("Cannot parse text").1
}

// Javscript export
#[wasm_bindgen]
pub fn parse_demo(input: &str) -> JsValue {
    JsValue::from_serde(&rust_parse_demo(input))
        .expect("Cannot deserialize Rust struct to Javascript")
}

#[wasm_bindgen]
pub fn format_content_demo(input: &str) -> JsValue {
    let parsed_content = rust_parse_demo(input);
    let mut formatted_content = String::new();

    for part in parsed_content.into_iter() {
        formatted_content.push_str(
            match part {
                TextParts::URI(uri) => {
                    let scheme = match uri.scheme {
                        Scheme::HTTP => "http://",
                        Scheme::HTTPS => "https://",
                    };
                    let host = match uri.host {
                        Host::HOST(x) => x,
                        Host::IPV4(arr) => arr.iter().map(|nb| nb.to_string()).collect::<Vec<String>>().join("."),
                        Host::IPV6(arr) => [
                            "[",
                            &arr.iter().map(|nb| nb.to_string()).collect::<Vec<String>>().join(":"),
                            "]",
                        ].concat(),
                    };
                    let port = match uri.port {
                        Some(nb) => [":", &nb.to_string()].concat(),
                        None => String::new(),
                    };
                    let path = uri.path.unwrap_or(String::new());
                    let query = uri.query.unwrap_or(String::new());

                    [scheme, &host, &port, &path, &query].concat()
                },
                TextParts::Mention(mention) => [
                    "<a href=\"#ToReplace-id-",
                    &mention.user_id.to_string(),
                    "\">@",
                    mention.name,
                    "</a>",
                ].concat(),
                TextParts::Other(part) => part.to_owned(),
            }.as_str()
        )
    }

    JsValue::from_str(&formatted_content)
}
