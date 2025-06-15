use std::fmt;
use std::str;

pub use self::Encoding::{Brotli, Chunked, Compress, Deflate, Ext, Gzip, Identity, Star, Trailers};

/// A value to represent an encoding used in `Transfer-Encoding`
/// or `Accept-Encoding` header.
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub enum Encoding {
    /// The `*` (= all) encoding.
    Star,
    /// The `chunked` encoding.
    Chunked,
    /// The `br` encoding.
    Brotli,
    /// The `gzip` encoding.
    Gzip,
    /// The `deflate` encoding.
    Deflate,
    /// The `compress` encoding.
    Compress,
    /// The `identity` encoding.
    Identity,
    /// The `trailers` encoding.
    Trailers,
    /// Some other encoding that is less common, can be any String.
    Ext(String),
}

impl fmt::Display for Encoding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Chunked => "chunked",
            Star => "*",
            Brotli => "br",
            Gzip => "gzip",
            Deflate => "deflate",
            Compress => "compress",
            Identity => "identity",
            Trailers => "trailers",
            Ext(ref s) => s.as_ref(),
        })
    }
}

impl str::FromStr for Encoding {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Encoding, crate::Error> {
        match s {
            "*" => Ok(Star),
            "chunked" => Ok(Chunked),
            "br" => Ok(Brotli),
            "deflate" => Ok(Deflate),
            "gzip" => Ok(Gzip),
            "compress" => Ok(Compress),
            "identity" => Ok(Identity),
            "trailers" => Ok(Trailers),
            _ => Ok(Ext(s.to_owned())),
        }
    }
}
