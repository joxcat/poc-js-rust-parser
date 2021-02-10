use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub enum TextParts<'a> {
    URI(URI),
    Mention(Mention<'a>),
    Other(&'a str),
}

// URI
#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct URI {
    pub scheme: Scheme,
    pub host: Host,
    pub port: Option<u16>,
    pub path: Option<String>,
    pub query: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub enum Scheme {
    HTTP,
    HTTPS,
}

impl From<&str> for Scheme {
    fn from(origin: &str) -> Self {
        match origin.to_lowercase().as_str() {
            "http://" => Scheme::HTTP,
            "https://" => Scheme::HTTPS,
            _ => unimplemented!("No other schemes supported for the moment"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub enum Host {
    HOST(String),
    IPV4([u8; 4]),
    IPV6([u16; 8]),
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub enum IPNum {
    IPV4(u8),
    IPV6(u16)
}

impl Into<Option<u8>> for IPNum {
    fn into(self) -> Option<u8> {
        match self {
            IPNum::IPV4(n) => Some(n),
            _ => None,
        }
    }
}

impl Into<Option<u16>> for IPNum {
    fn into(self) -> Option<u16> {
        match self {
            IPNum::IPV6(n) => Some(n),
            _ => None,
        }
    }
}

// Mention
#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Mention<'a> {
    pub user_id: u32,
    pub name: &'a str,
}
