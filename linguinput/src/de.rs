use std::fmt;

use thiserror::Error;
use unicode_segmentation::UnicodeSegmentation;

use crate::{table::Table, TableInitError};

#[derive(Debug, Error)]
pub enum DecodingError {
    #[error("{}", .0)]
    TableInit(
        #[from]
        #[source]
        TableInitError,
    ),
    #[error("Error formatting encoded data")]
    Fmt(
        #[source]
        #[from]
        fmt::Error,
    ),
}

#[derive(Debug, Clone)]
pub struct Decoder<W> {
    table: &'static Table,
    target: W,
}

impl<W> Decoder<W>
where
    W: fmt::Write,
{
    pub fn new(target: W) -> Result<Self, DecodingError> {
        Ok(Self { table: Table::load()?, target })
    }

    pub fn push(
        &mut self,
        ch: &'static str,
    ) -> Result<&mut Self, DecodingError> {
        match self.table.char_to_code(ch) {
            Some(code) => write!(self.target, "{}", code)?,
            None => write!(self.target, "{}", ch)?,
        }
        Ok(self)
    }

    pub fn push_str(
        &mut self,
        content: &str,
    ) -> Result<&mut Self, DecodingError> {
        for ch in content.graphemes(true) {
            self.push_str(ch)?;
        }
        Ok(self)
    }
}
