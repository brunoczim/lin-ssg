use std::fmt;

pub use de::{Decoder, DecodingError};
pub use en::{
    Display,
    DisplayAdapter,
    DisplayFormat,
    Encode,
    Encoder,
    EncodingError,
};
pub use table::TableInitError;

mod table;
mod en;
mod de;

#[cfg(test)]
mod test;

pub fn encode(input: &str) -> Result<String, EncodingError> {
    let mut buf = String::new();
    encode_to(input, &mut buf)?;
    Ok(buf)
}

pub fn encode_to<W>(input: &str, target: W) -> Result<(), EncodingError>
where
    W: fmt::Write,
{
    let mut encoder = Encoder::new(target)?;
    encoder.push_str(input)?;
    encoder.finish()?;
    Ok(())
}

pub fn decode(input: &str) -> Result<String, DecodingError> {
    let mut buf = String::new();
    decode_to(input, &mut buf)?;
    Ok(buf)
}

pub fn decode_to<W>(input: &str, target: W) -> Result<(), DecodingError>
where
    W: fmt::Write,
{
    let mut decoder = Decoder::new(target)?;
    decoder.push_str(input)?;
    Ok(())
}
