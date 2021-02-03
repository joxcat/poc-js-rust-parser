// URI
#[derive(Debug, PartialEq, Eq)]
pub struct URI<'a> {
    scheme: Scheme,
    host: Host,
    port: Option<u16>,
    path: Option<Vec<&'a str>>,
    query: Option<QueryParams<'a>>,
}

#[derive(Debug, PartialEq, Eq)]
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


#[derive(Debug, PartialEq, Eq)]
pub enum Host {
    HOST(String),
    IPV4([u8; 4]),
    IPV6([u16; 8]),
}

pub type QueryParam<'a> = (&'a str,  &'a str);
pub type QueryParams<'a> = Vec<QueryParam<'a>>;

// Mention
#[derive(Debug, PartialEq, Eq)]
pub struct Mention<'a> {
    user_id: u32,
    name: &'a str
}
