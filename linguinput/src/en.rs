use std::fmt::{self, Write};

use thiserror::Error;

use crate::{table::Table, TableInitError};

#[derive(Debug, Error)]
pub enum EncodingError {
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
    #[error("Unmatched '{{'")]
    UnmatchedOpen,
    #[error("Unmatched '}}'")]
    UnmatchedClose,
    #[error("Code {} is too big", .0)]
    CodeTooBig(String),
    #[error("Unknown code {}", .0)]
    UnknownCode(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum EncoderState {
    Default,
    Opening,
    Closing,
}

impl Default for EncoderState {
    fn default() -> Self {
        Self::Default
    }
}

#[derive(Debug, Clone)]
pub struct Encoder<W> {
    table: &'static Table,
    buf: String,
    state: EncoderState,
    target: W,
}

impl<W> Encoder<W>
where
    W: fmt::Write,
{
    pub fn new(target: W) -> Result<Self, EncodingError> {
        let table = Table::load()?;
        Ok(Self {
            table,
            state: EncoderState::Default,
            buf: String::with_capacity(table.max_code_len()),
            target,
        })
    }

    pub fn push(&mut self, ch: char) -> Result<&mut Self, EncodingError> {
        match self.state {
            EncoderState::Default if ch == '{' => {
                self.state = EncoderState::Opening;
            },
            EncoderState::Default if ch == '}' => {
                self.state = EncoderState::Closing;
            },
            EncoderState::Default => {
                write!(self.target, "{}", ch)?;
            },
            EncoderState::Opening if self.buf.is_empty() && ch == '{' => {
                write!(self.target, "{}", ch)?;
                self.state = EncoderState::Default;
            },
            EncoderState::Opening if ch == '}' => {
                let Some(encoded) = self.table.code_to_char(&self.buf) else {
                    Err(EncodingError::UnknownCode(self.buf.clone()))?
                };
                write!(self.target, "{}", encoded)?;
                self.buf.clear();
                self.state = EncoderState::Default;
            },
            EncoderState::Opening
                if self.buf.len() + ch.len_utf8()
                    > self.table.max_code_len() =>
            {
                let mut code = self.buf.clone();
                code.push(ch);
                Err(EncodingError::CodeTooBig(code))?;
            },
            EncoderState::Opening => {
                self.buf.push(ch);
            },
            EncoderState::Closing if self.buf.is_empty() && ch == '}' => {
                write!(self.target, "{}", ch)?;
                self.state = EncoderState::Default;
            },
            EncoderState::Closing => {
                Err(EncodingError::UnmatchedClose)?;
            },
        }

        Ok(self)
    }

    pub fn push_str(
        &mut self,
        content: &str,
    ) -> Result<&mut Self, EncodingError> {
        for ch in content.chars() {
            self.push(ch)?;
        }
        Ok(self)
    }

    pub fn encode<T, F>(
        &mut self,
        target: T,
        format: F,
    ) -> Result<&mut Self, EncodingError>
    where
        F: Copy,
        T: Encode<F>,
    {
        target.encode(format, self)?;
        Ok(self)
    }

    pub fn write_fmt(
        &mut self,
        arguments: fmt::Arguments,
    ) -> Result<(), EncodingError> {
        struct Adapter<'a, W> {
            encoder: &'a mut Encoder<W>,
            result: Result<(), EncodingError>,
        }

        impl<'a, W> fmt::Write for Adapter<'a, W>
        where
            W: fmt::Write,
        {
            fn write_str(&mut self, content: &str) -> fmt::Result {
                if self.result.is_err() {
                    Err(fmt::Error)?;
                }
                self.encoder.push_str(content).map_err(|error| {
                    self.result = Err(error);
                    fmt::Error
                })?;
                Ok(())
            }
        }

        let mut adapter = Adapter { encoder: self, result: Ok(()) };
        let _ = adapter.write_fmt(arguments);
        adapter.result
    }

    pub fn finish(&mut self) -> Result<(), EncodingError> {
        match self.state {
            EncoderState::Default => Ok(()),
            EncoderState::Opening => Err(EncodingError::UnmatchedOpen),
            EncoderState::Closing => Err(EncodingError::UnmatchedClose),
        }
    }
}

pub trait Encode<F>
where
    F: Copy,
{
    fn encode<W>(
        &self,
        format: F,
        encoder: &mut Encoder<W>,
    ) -> Result<(), EncodingError>
    where
        W: fmt::Write;

    fn render_encoded(&self, format: F) -> Result<String, EncodingError> {
        let mut buf = String::new();
        let mut encoder = Encoder::new(&mut buf)?;
        self.encode(format, &mut encoder)?;
        Ok(buf)
    }
}

impl<'a, T, F> Encode<F> for &'a T
where
    F: Copy,
    T: Encode<F> + ?Sized,
{
    fn encode<W>(
        &self,
        format: F,
        encoder: &mut Encoder<W>,
    ) -> Result<(), EncodingError>
    where
        W: fmt::Write,
    {
        (**self).encode(format, encoder)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DisplayFormat;

#[derive(Debug, Clone, Copy)]
pub struct Display<T>(pub T);

impl<T> Encode<DisplayFormat> for Display<T>
where
    T: fmt::Display,
{
    fn encode<W>(
        &self,
        _format: DisplayFormat,
        encoder: &mut Encoder<W>,
    ) -> Result<(), EncodingError>
    where
        W: fmt::Write,
    {
        write!(encoder, "{}", self.0)
    }
}

pub struct DisplayAdapter<T, F>(pub T, pub F);

impl<T, F> Encode<DisplayFormat> for DisplayAdapter<T, F>
where
    F: Copy,
    T: Encode<F>,
{
    fn encode<W>(
        &self,
        _format: DisplayFormat,
        encoder: &mut Encoder<W>,
    ) -> Result<(), EncodingError>
    where
        W: fmt::Write,
    {
        self.0.encode(self.1, encoder)
    }
}
