use std::{borrow::Cow, fmt, str};

/// A value to represent an encoding used in `Transfer-Encoding`
/// or `Accept-Encoding` header.
#[derive(Clone, PartialEq, Debug)]
pub enum Encoding {
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
    Ext(Cow<'static, str>),
}

impl fmt::Display for Encoding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Encoding::Chunked => "chunked",
            Encoding::Brotli => "br",
            Encoding::Gzip => "gzip",
            Encoding::Deflate => "deflate",
            Encoding::Compress => "compress",
            Encoding::Identity => "identity",
            Encoding::Trailers => "trailers",
            Encoding::Ext(ref s) => s.as_ref(),
        })
    }
}

impl str::FromStr for Encoding {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Encoding, crate::Error> {
        match s {
            "chunked" => Ok(Encoding::Chunked),
            "br" => Ok(Encoding::Brotli),
            "deflate" => Ok(Encoding::Deflate),
            "gzip" => Ok(Encoding::Gzip),
            "compress" => Ok(Encoding::Compress),
            "identity" => Ok(Encoding::Identity),
            "trailers" => Ok(Encoding::Trailers),
            _ => Ok(Encoding::Ext(Cow::Owned(s.to_owned()))),
        }
    }
}
